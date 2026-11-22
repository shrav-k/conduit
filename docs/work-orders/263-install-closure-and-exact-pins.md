# Work order 263: `conduit install` installs the archive's missing closure, then pins it exactly

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`;
leave `vectors` empty. This order needs no network: its only sockets are its tests' loopback
ports. Run cargo offline, one command at a time; if a build asks for a crate not in the cache,
stop and report. Never print a whole large file, diff or log. Never run `env`, `printenv`, `set`
or `export -p`, never print an environment variable's value, and never read or print a key, a
token or any credential. Do not run `conduit` outside a test, `npm`, `codex`, `claude`, the
dogfood harness or `tools/gate.py`, and sign in nowhere. Prefix each `python3` with
`PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute local path, user,
machine or private repository name.

## What and why

**The owner's answers** ([DEC-048](../decisions/log.md), citing DEC-040): the model -> "Exact
digests + closure walk (Recommended)"; install's pin -> "Exact, up or down (Recommended)";
dependencies -> "Install the closure (Recommended)"; sources -> "Cache, file: fetch, mirrors
(Recommended)"; S1's T2 -> "As written (Recommended)"; https -> "Nothing remote in v0
(Recommended)". Today, after order 243, `install` caches the archive, sends one `install.v1`
and writes no pin (243's decision 6, test `install_changes_no_manifest`); PKG-028 refuses an
archive listing a package not installed, so S1's T2 installs history's dependency by hand.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 6:
each package is installed by its own printed request, and only the named pin moves; 5: the
output tells an agent what was installed, for whom, from where, and what stopped. None at risk;
a dependency installed can widen an ACC-029 bound, as any install can (PKG-031).

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)).
Yes: `npm ci` and `terraform init` install the recorded closure; `go get` records the version
named, up or down; Go's module cache "may be used as a proxy ... with a file:// URL"; PKG-006
lets the CLI offer "one command that installs them". Not taken: version selection, ranges and a
solver (each rewrites NAM-014); `conduit add`; pins moving only up; a dependencies flag; a mirror
by address and version; CLI https.

**When it runs.** After 243's build and DEC-048 merge, beside 245, before 250, through Conduit's
dogfood: one Codex worker turn (4-hour limit); a Codex verifier and a Claude reader at once,
blind, running only what settles a doubt; one fix turn; the gate (DEC-046, DEC-033). By ruling
89, each decision says what must hold; how is yours. The forms, statuses, names and
`config.json`, and the readings that a pin holding `fetch` stops the run, that `--dry-run`
sends `list.v1`, are the coordinator's, open to the owner's strike.

**Shared files.** `main.rs` (245, 247 to 249: argument loop, command lists), `help.rs` (245, 247
to 250: arrays, lengths, footer), `lib.rs` (245), `engine/README.md` (250) and K3's README (246,
250). Touch only: in `main.rs`, install-only arms for `--mirror` and `--dry-run` beside `--as`'s,
with their variables and `author::Options` fields; in `help.rs`, install's tuple and
description; in `lib.rs`, one module line; in K3, decision 8's.

**Stop rule.** Stop, change nothing further, and report where a start-check line prints
otherwise; a wording needs a choice DEC-048 and `spec/` leave open; a result needs a change
outside the scope (the core, the loader, a case, `packages/`, `engine/CLI.md`, a `Cargo.toml` or
a lockfile); the engine refuses a request of the walk it accepts sent by hand, as filesystem
0.1.0 beside 0.1.1 at T2 might (order 250's decision 8): keep the test failing, report request
and reply; `vector_delta.py` reports a change; or a crate or the network is needed.

A choice no decision governs is yours: list it under `own_choices`.

## The decisions

1. **The spec text** (DEC-048). No engine behaviour changes.
   - PKG-028's `observable`, after "each is installed first, by a request of its own.", gains:
     "As under PKG-006, the reference engine's command line may offer one command that installs
     them first, through every dependency and each by a request of its own, which is no promise."
   - NAM-012's `observable`, after "the place the item gives.", gains: "Where the reference
     engine's command line takes the archive from a cache or from a mirror that the user names,
     it picks that copy before it sends the request and gives its `file:` URL in `fetch`, once the
     copy's bytes hash to the item's digest; how the user names a mirror is DL7."
   - `spec/README.md`: "Package pins belong to the manifest, and `conduit install` writes them
     into the author's file." becomes "Package pins belong to the manifest: `conduit install
     ARCHIVE#HEX` writes the given archive's pin into the author's file, at exactly its version
     and digest, after installing what that archive lists, and pins no dependency."
   - `spec/CHANGES.md`, under `## [Unreleased]`:

     ```text
     - Changed: PKG-028's and NAM-012's observables and `spec/README.md#an-author-of-documents`: the reference CLI may install an archive's missing dependencies first, each by its own request, from a cache or mirror copy it picks, and pins only that archive, exactly (DEC-048).
     ```

2. **The walk** (PKG-028, NAM-014, PKG-032). `install ARCHIVE#HEX` hashes the archive as today and
   walks its `packages` items, then each found archive's, never the core (NAM-021), each digest
   once. Where a walked manifest lists a package, it first sends `management.list.v1` as root,
   once; a package listed there at that version and digest is *present*, not walked below. Each
   other gets its own `install.v1`, one at a time, depth first, items in written order, the named
   archive last and always. An item without a version and a `sha256:` digest is the engine's to
   judge. No `confirmClientIds` is sent. `boundary::install_arguments` stays.

3. **Sources** (NAM-012, PKG-021, PKG-026, DL7). For each package not present, the first place
   whose bytes hash to its digest: the cache; the item's `fetch`, if an absolute `file:` URL; each
   mirror, a directory in the cache's layout (files named by lowercase hex digest): each
   `--mirror DIR` (repeatable, relative to the current directory), then each in `config.json` in
   243's per-user directory, whose one member `mirrors` lists absolute directories; nothing of
   Conduit's writes it. A mirror that is not a directory, or another `config.json` form, stops the
   run first. An https or relative `fetch` is a miss (unbuilt in v0). A request's `fetch` is the
   `file:` URL of the place read. Each archive whose bytes hash to its digest is cached, as 243
   caches the named one, even if the run then stops; `--dry-run` caches none. `--store` changes
   none of this.

4. **Output and stops** (ruling 52). On stdout: each `present` line; per request, its `install`
   line, then the engine's reply as one JSON line, as on `main`; then the `pin` or `stop` lines.
   Where any of these holds before the first `install.v1`, the walk sends none and exits 2, a `stop`
   line per cause: a package in no place (every branch walked, every miss named); an address and
   version, the named archive's included, installed or listed with another digest, or twice in the
   closure (PKG-025); a found manifest naming another address or version (PKG-023); decision 5's pin
   not writable; what the archives in hand fail of NAM-024, PKG-029 and CTR-013, checked by the
   core's own code (none written twice). The rest, and a package's own rejection, are the engine's:
   after a sent request that does not succeed, the walk sends no more, writes no pin, prints `not
   sent [ADDRESS@VERSION,...]` and a stderr line naming that request, what this run installed and
   what not, and that nothing was rolled back; exit 1. `--dry-run` sends list.v1 alone and prints
   the lines but the replies, then `dry-run: no install.v1 sent, nothing written`.

   ```text
   present Package/ADDRESS version=VERSION digest=sha256:HEX needed-by=[NEEDER,...]
   install Package/ADDRESS version=VERSION digest=sha256:HEX from=SOURCE needed-by=[NEEDER,...]
   pin added ADDRESS version=VERSION digest=sha256:HEX file=FILE
   pin moved up ADDRESS version=OLD->VERSION digest=sha256:OLDHEX->sha256:HEX file=FILE
   stop Package/ADDRESS version=VERSION digest=sha256:HEX CAUSE needed-by=[NEEDER,...] PROMISE
   stop Manifest/- file=FILE cause=CAUSE pin=ADDRESS@VERSION digest=sha256:HEX requires=edit -
   ```

   Also `pin unchanged`; `pin moved down` or, a digest alone, `pin moved` (semver precedence);
   `pin none: no manifest in DIR`. CAUSE: `archive=absent tried=[PLACE,...]` (PKG-028; PKG-006
   for a restored pin), `installed=` or `listed=sha256:OTHER` (PKG-025),
   `manifest=ADDRESS@VERSION` (PKG-023). NEEDER: `ADDRESS@VERSION` of each lister, or
   `Manifest/-`, sorted; `[]` for the named archive. SOURCE: `argument`, `cache`, `fetch:URL` or
   `mirror:DIR` (DIR as given); a PLACE is a SOURCE, plus `(mismatch)` or `(unbuilt)`. Each
   stop's stderr line says nothing was sent, and why.

5. **The pin** (DEC-048; NAM-006, MAN-001). Once the named archive's request succeeds, `install`
   writes its pin, its version and digest, up or down, into the one MAN-001 file in the current
   directory, atomically, keeping the mode: never a dependency's, never a `fetch`; an unchanged
   pin is not written; no MAN-001 file: `pin none`. The bytes are computed before any request: one
   span replaced (the item, else an insertion after the last item or for an empty `{}`), every
   other byte kept, the loader reading only that item changed. It must so edit 243's `packages:
   {}`; a block map of one-line items `ADDRESS: {version: V, digest: 'sha256:HEX'}`, a new YAML
   item's form; a block map of block items of `version` and `digest` (S1's); and `conduit.json`.
   Another layout, two MAN-001 files, a manifest the loader rejects, or an item holding `fetch`
   is a `stop Manifest/-`. A file changed since that check is left, saying so.

6. **Restoring and statuses.** `conduit install` with no archive walks every pin of the manifest
   in the current directory, each needed by `Manifest/-`, with `--mirror` and `--dry-run`, and
   writes no manifest and no `pin` line; with no manifest it exits 2, naming MAN-001's three
   names. Exit 0 where every request sent succeeded and nothing stopped; 1 where one did not, or
   its transport failed; 2 for a stop before an `install.v1`, the argument's own read and digest
   errors among them, and for a pin left after the requests.

7. **Tests**, in a new `engine/crates/shell/tests/install_closure.rs`, against the binary, each
   child with its own store, port, `CONDUIT_CONFIG_DIR` and current directory, archives built as
   `tests/sections.rs` builds one; write them first and report each one's failing line:
   - `install_walks_the_closure_leaves_first`: A lists B lists C: C, B, A, with `from=` and
     `needed-by=`; cache, `fetch:`, `--mirror`, `config.json` in turn, a mismatched copy passed
     over, with and without `--store`; all then cached; run again, B and C `present`; each bad
     mirror form exits 2.
   - `install_pins_exactly_up_or_down`: each layout of decision 5 added, moved up, down and
     unchanged, other bytes kept; no dependency pinned; `pin none`; each `stop Manifest/-`.
   - `install_stops_before_any_request_on_a_miss_or_conflict`: `missing-dependency.tar.gz` of
     `install-archives/`; https and relative `fetch`; misses in two branches; PKG-025 both ways;
     another address: exit 2, packages unchanged.
   - `install_stops_partway_and_names_what_it_installed`: A lists `agents-0.2.1.tar.gz` of
     `s1/archives/`, then filesystem 0.1.0 at `invalid-document.tar.gz`'s digest in a mirror:
     exit 1, agents installed, A `not sent`.
   - `install_dry_run_and_restoring_write_nothing`: `--dry-run` changes no package, cache file
     or manifest; restoring pins from a mirror leaves the manifest; no manifest: exit 2.
   - `s1_t2_is_one_install_line`: in an empty directory with `conformance` linked to the
     repository's, bare `init`, then K3's T1 install lines and T2's, read from K3 and run as
     written: T2 installs filesystem 0.1.0 `from=mirror:conformance/fixtures/s1/mirror`, needed by
     `example.org/s1/history@0.1.0`, then history, adding history's pin alone.

   Delete 243's `install_changes_no_manifest`. Change an existing test only where it reads
   `install`'s stdout or status, or where an install now pins into a manifest it reads (give
   that child an empty directory); list each.

8. **The S1 mirror and K3.** Add `conformance/fixtures/s1/mirror/` holding
   `archives/filesystem.tar.gz`'s bytes named by its digest, `200de7c1…2160` in full, and a
   section `## The mirror` of the fixtures' README saying so and that no case reads it. In K3's
   README, these alone:
   - T2's two install lines become one, `T2: conduit install --mirror
     conformance/fixtures/s1/mirror
     conformance/fixtures/s1/archives/history.tar.gz#e946746509490be49cbcc07d41ff5fed24a6e6c8b377bf423c77f79c02502f69`;
   - the counts' rows become `| T2 | 1 | 2 | 3 | 0 | 4 | 5 |` and
     `| Total | 8 | 12 | 26 | 2 | 34 | 40 |`;
   - "is installed explicitly at T2, before history;" becomes "is installed at T2 by history's
     own install line, from the mirror fixture, before history;", and the sentence "This
     resolves the old O6 accounting question ..." goes;
   - the sequences become `21, 4, 0, 3, 2, 2, 2` and strict `25, 5, 0, 3, 2, 2, 3`; the paragraph
     says they were `21, 5, 0, 3, 2, 2, 2` and `25, 6, 0, 3, 2, 2, 3` before DEC-048, so its
     agreement with experiment 023 and ruling 76 is gone, T2's line being a tooling change
     (ruling 71 point 4, linked pinned as the page links rulings);
   - O7's "keeps a copy of the archive" says each archive it reads whose bytes hash to its
     digest, the given one and each dependency.

   Every case, 01, 02 and 05 among them, keeps its bytes.

9. **The documents.** `help.rs`: install's usage `[ARCHIVE#SHA256] [--mirror DIR] [--dry-run]`,
   decisions 2, 3, 5 and 6 in its description; `engine/README.md`'s install sentences likewise,
   briefly; neither says install writes no pin. `engine/CONFORMANCE.md`: DL7's storage row ends
   "From order 263, the command line reads mirrors (`--mirror DIR`, the per-user `config.json`)
   beside order 243's package cache, directories of archives named by digest, and hands the
   engine a `file:` URL of the copy (NAM-012)."; DL7's unfinished row says "Package removal, an
   engine-side mirror," for "Package removal/mirror controls,". Leave `docs/decisions/` alone.

## What to read

PKG-006, PKG-021 to PKG-028, NAM-006, NAM-011 to NAM-014, MAN-001, DL7; 243's install path; K3's
O7 and S1 schedule.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor 14ee5074 HEAD; echo "main $?"
sed -n '/^## DEC-048/,/^## DEC-049/p' docs/decisions/log.md | grep -c '^Status: accepted, not done'
K=docs/implementation/plan-and-apply/README.md
grep -c '^O8: Decided' $K
grep -c 'fn install_changes_no_manifest' engine/crates/shell/tests/context.rs
grep -c '^T2: conduit install' $K
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
j spec/promises/packages.yaml 'each is installed first, by a request of its own.'
j spec/promises/naming.yaml 'the place the item gives.'
j spec/README.md "writes them into the author's file."
ls conformance/fixtures/s1/mirror engine/crates/shell/tests/install_closure.rs 2>/dev/null | wc -l | tr -d ' '
test -e "$HOME/.config/conduit"; echo "home $?"
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
git status --short | wc -l | tr -d ' '
```

They print `main 0`; `1` once DEC-048 is logged; `1` and `1` once 243's build has merged; `2`,
`1`, `1`, `1`, `0`, `home 1`, `engine 0` and `0`.

## Scope

`engine/crates/shell/src/` (a new module, `author.rs`, `boundary.rs`, and as "Shared files"
says), `engine/crates/shell/tests/` (as decision 7 allows), decision 1's four `spec/` files,
`engine/README.md`, `engine/CONFORMANCE.md`, `engine/STANDING.md` by the tool alone, K3's
README, `conformance/fixtures/s1/README.md` and the new mirror file. Nothing else.

## Done when

Each line of the first group prints otherwise with 243 merged ("today") and passes after.

```sh
(cd engine && cargo test --offline --release -p conduit-shell --test install_closure 2>&1 | grep -cE '^test (install_(walks_the_closure_leaves_first|pins_exactly_up_or_down|stops_before_any_request_on_a_miss_or_conflict|stops_partway_and_names_what_it_installed|dry_run_and_restoring_write_nothing)|s1_t2_is_one_install_line) \.\.\. ok$')  # 6; today 0 (no such target)
grep -c 'fn install_changes_no_manifest' engine/crates/shell/tests/context.rs  # 0; today 1
F=conformance/fixtures/s1; cmp -s $F/archives/filesystem.tar.gz $F/mirror/200de7c103aa560201dab8821e974088ce470e76c859c86aa01605cdf72d2160; echo "mirror $?"  # mirror 0; today mirror 2
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
j spec/promises/packages.yaml "may offer one command that installs them first, through every dependency"  # 1; today 0
j spec/promises/naming.yaml "it picks that copy before it sends the request"  # 1; today 0
j spec/README.md "writes the given archive's pin into the author's file, at exactly its version"  # 1; today 0
sed -n '/^## \[Unreleased\]/,$p' spec/CHANGES.md | grep -c "^- Changed: PKG-028's .*(DEC-048)\.$"  # 1; today 0
K=docs/implementation/plan-and-apply/README.md
grep -c '^T2: conduit install --mirror conformance/fixtures/s1/mirror conformance/.*/history\.tar\.gz#e9467465' $K  # 1; today 0
grep -c '^T2: conduit install' $K  # 1; today 2
grep -c -e '^| T2 | 1 | 2 | 3 | 0 | 4 | 5 |$' -e '^| Total | 8 | 12 | 26 | 2 | 34 | 40 |$' $K  # 2; today 0
grep -oE '21, 4, 0, 3, 2, 2, 2|25, 5, 0, 3, 2, 2, 3' $K | sort -u | wc -l | tr -d ' '  # 2; today 0
grep -l -- '--mirror' engine/crates/shell/src/help.rs engine/README.md | wc -l | tr -d ' '  # 2; today 0
grep -c 'From order 263,' engine/CONFORMANCE.md  # 1; today 0
```

These pass today and must still pass; run the whole suite once, last:

```sh
git add -N engine conformance/fixtures/s1/mirror  # so the checks see the new files
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-shell --all-targets -- -D warnings)
PYTHONDONTWRITEBYTECODE=1 /usr/bin/python3 tools/vector_delta.py HEAD WORKTREE >/dev/null; echo "delta $?"  # delta 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_docs.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/promise_standing.py --check  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_private_paths.py  # Errors: 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(engine/crates/shell/(src|tests)/|engine/(README|CONFORMANCE|STANDING)\.md$|spec/(promises/(packages|naming)\.yaml|README\.md|CHANGES\.md)$|docs/implementation/plan-and-apply/README\.md$|conformance/fixtures/s1/(README\.md|mirror/[0-9a-f]{64})$)' | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release)  # the whole suite, once, last
test -e "$HOME/.config/conduit"; echo "home $?"  # home 1
git reset -q  # stage nothing
```

Report per decision what changed, each new test's failing and passing lines, each test changed,
and every check's last line. Stage nothing.

## What follows

- One walk for plan, apply and install; https with the first remote root (DEC-048); `conduit
  mirror`; removing a package nothing holds.
- Orders 246 and 250, amended for DEC-048 before they run.

## Record

Not yet run.
