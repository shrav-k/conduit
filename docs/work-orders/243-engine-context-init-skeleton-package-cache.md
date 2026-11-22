# Work order 243: the engine context that `conduit serve` records, `init`'s skeleton, and `install`'s package cache (pins wait for a resolver)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty, since no case changes. This order needs no
network: it reaches no host and fetches no crate, and the only sockets it opens are the loopback
ports its tests bind. Every crate it builds is in the local cache already; if a build asks for one
that is not, stop and report, and fetch nothing. Run one cargo command at a time. Never print a
whole large file, diff, fixture or log: use `head`, `tail`, `grep`, `sed -n` or a count. Never run
`env`, `printenv`, `set` or `export -p`, never print an environment variable's value, and never
read or print a key, a token or any credential, even one a test made. Do not run `conduit` outside
a test, `codex`, `claude` or the dogfood harness, and sign in nowhere. Every file you write is
public: no absolute local path, no user or machine name and no private repository name.

## What and why

**The gap.** The S1 rerun counts a command only if it carries a value the author chose
([ruling 63, point 1](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-63/README.md)).
[K3's schedule](../implementation/plan-and-apply/README.md#s1-commands-and-counts-under-ruling-76)
has a bare `conduit init`, `conduit install ARCHIVE#HEX` and a bare `conduit apply`. On `main`
every command needs `--store DIR`, `init` writes no manifest and `install` keeps no copy of its
archive. `spec/README.md` promises the skeleton, and DEC-008's wording (order 240) the context
([ruling 76, point 3](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-76/README.md)).
K3's O7 decided the package cache that `plan` reads (order 245) and left its place to O8, still
open. This order closes O8 and builds the context, the skeleton and the cache. It writes no pin
into the author's manifest: by the coordinator's reading of answer 1b, pins wait for a resolver
(decision 6).

**Two readings that differ from the chain's plan** (the coordinator's, open to the owner's strike):

- *`init` has two roles, chosen by `--store`.* Bare `conduit init` writes the skeleton and opens no
  store; `conduit init --store DIR` initializes that store as today and writes nothing outside it.
  K3's T1 `init` runs after `serve` (harness) has recorded the context, so the store exists. The
  tests and the dogfood run `init --store` in directories that are not projects.
- *`serve` and `register` keep `--store`.* A bare `serve` would start whatever store the context
  last named, the dogfood's among them; `register` opens the store itself with `serve` stopped,
  so no context engine can answer for it. Neither is on S1's schedule.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes,
as existing practice. kubectl keeps its current context in `~/.kube/config`, relocated by
`KUBECONFIG`; Docker keeps `currentContext` in `~/.docker/config.json`, relocated by
`DOCKER_CONFIG`; gh keeps `~/.config/gh`, relocated by `GH_CONFIG_DIR`. `npm init` and `cargo init`
write a skeleton manifest. npm's cache keys its content by integrity hash, and Go's module
cache and Cargo's registry cache keep downloaded archives outside every project; a cache keyed by
digest is a building block a resolver reads from. Not taken: a context-selection command (K3
proposes none); context in the project's `.conduit/` (O8 keeps it out); a rule of this order's
own for writing or moving pins (answer 1b).

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): serves
6 in part: a stale context stops before any request, and each bare command names on stderr the
store and URL it reaches, so none reaches an engine silently. None at risk: no promise, case or
operation changes; the context holds no credential (the client token stays in the store), and
the cache holds archives by digest.

**The owner's items 1a and 1b.** 1a: what a bare `conduit init` writes (decision 4). 1b: the
rule by which `install` writes or moves a pin in the author's manifest (decision 6). The owner
answered both on 2026-11-21, at the post-dogfood review.
Answer 1a: "Skeleton manifest only (Recommended)": `apiVersion` and `packages: {}` in flow
style, nothing else; where a manifest exists, `init` changes nothing and says so.
Answer 1b: "We should research resolvers as this is a key part of conduit, we should not build
hacks around this". The coordinator reads this, open to the owner's strike, as: this order
writes and moves no pin; pins come with the resolver design.

**When it runs.** After orders 240 and 241 merge (the start check says). Through Conduit's
dogfood: one Codex worker turn (4-hour limit, aim for under 2; the whole suite once, last), a
Codex verifier and a Claude reader, who run only what settles a doubt, and the gate
([DEC-033](../decisions/log.md#dec-033-2026-11-20-the-testing-audit)). By
[ruling 89](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-89/README.md),
each decision says what must hold; how is yours. The variable's name, the file layout and the test
names are the coordinator's, open to the owner's strike.

**Stop rule.** Change nothing under `spec/`. Stop, change nothing further, and report where:

- a start-check line prints otherwise;
- a correct result needs a change outside the scope: the loader, the core, `conformance/`,
  `packages/`, a K3 case, or a crate not already in `engine/Cargo.lock`;
- a test or a step needs `install` to read or write the author's manifest (decision 6);
- a crate is missing from the cache, or a step needs the network.

A choice no decision governs is yours: list it under `own_choices`.

## The decisions

1. **The per-user directory.** `CONDUIT_CONFIG_DIR` where set and not empty, else
   `$HOME/.config/conduit`. Either must be absolute. One function locates it, so that 245 and 247
   reuse it. It is made at mode 0700 when first written, and holds `context.json` and `packages/`.
   Nothing in a project's directory or a store points to it.
2. **`serve` records the context.** Once listening, before its ready line, `serve` replaces
   `context.json` atomically, at mode 0600, with exactly `store` (the canonical path its ready line
   prints), `url` (the ready line's URL) and `version` (the line `conduit --version` prints). The
   ready line is unchanged. The last `serve` started is current; none removes it. Only the
   `conduit serve` command writes it, not the library's `server::serve`, which unit tests and
   241's harness call in-process. Where the directory cannot be located or written, `serve`
   prints one stderr line naming the place and the cause, and serves as today.
3. **Commands use the context when `--store` is absent:** `install`, `submit`, `replace`,
   `retire`, `unregister`, `inspect`, `pending`, `answer`, `sign-in`, `records`, `tools` and
   `call` (and 245's `plan` and 247's `apply`, through the same function). With `--store`, a
   command reads no context and behaves as on `main`, but for `install`'s cache (decision 5).
   Without it, before it sends or opens anything, it stops with
   **status 2**, writing nothing, with one stderr line that names the file and says to start
   `conduit serve --store DIR` or give `--store DIR` (ruling 52), where:
   - the directory cannot be located: `CONDUIT_CONFIG_DIR` is set and relative, or it is unset
     or empty and `HOME` is unset, empty or relative (name the variable at fault);
   - `context.json` is missing, or is not an object of the three strings, `store` absolute;
   - `store` is not an initialized store, or the URL it fixes is not the context's `url` (name
     the store);
   - no engine answers at that URL as that store's root, by the check `author::run` already makes
     before its work (name the store and the URL). A bare `inspect` or `records` never opens the
     store itself, as it does with `--store` when `serve` is stopped.

   Where the context passes, the command prints one stderr line naming the context file, the
   store and the URL, then runs as with `--store STORE`. The context's `version` is recorded for
   245 and 247, which compare it; this order compares no version. `init`, `serve` and `register`
   read no context: `serve` and `register` without `--store` refuse as today, and bare `init` is
   decision 4's.
4. **`init`'s skeleton (the owner's item 1a, answered).** Bare `conduit init` writes
   `conduit.yaml` in the current directory, two lines: `apiVersion: V`, V being `spec/names.yaml`'s
   `apiVersion` as the loader reads it, not a second literal, and `packages: {}`. Nothing else:
   no Pool, Fleet or Conduit (answer 1a). Where the directory already holds `conduit.yaml`,
   `conduit.yml` or `conduit.json` (MAN-001), it changes nothing and says so on stderr; exit 0
   either way. It reads no context, opens no store and sends nothing. `--label`,
   `--trust-domain`, `--url` and the runner options in help's footer are refused without `--store`.
   `init --store DIR` writes nothing outside `DIR`.
5. **The package cache (K3's O7).** Once the archive's bytes hash to HEX (today's check),
   `install` keeps them as `packages/HEX` (lowercase), at mode 0600, written atomically, whatever
   `install.v1` then answers; a file already there with other bytes is replaced. Nothing else
   writes the cache. Where the directory cannot be located or written, `install` prints one stderr
   line naming the place and the cause and saying the cache was not written, and goes on; a bare
   `install` with no directory located has already stopped by decision 3. `install.v1` itself is
   unchanged: its request, its answer, the one result line on standard output and the exit
   status are as on `main`. The cache is the building block a resolver will read; this order
   adds nothing that chooses a release.
6. **No pin (the coordinator's reading of answer 1b).** `install` reads, creates and changes no
   author's manifest (MAN-001's names), and so writes and moves no pin: not in the current
   directory, not anywhere, with or without `--store`, whatever `install.v1` answers. It prints
   nothing about pins. The author writes each pin, as on `main`; pin writing comes with the
   resolver design (What follows).
7. **The tests,** in a new `engine/crates/shell/tests/context.rs`, against the binary, each with
   its own temporary store, port and `CONDUIT_CONFIG_DIR`, at least these by these names:
   - `serve_records_the_engine_context`: decision 2's file, its three members alone, its modes,
     `version` as `conduit --version` prints it; a second store's `serve` replaces it; a relative
     `CONDUIT_CONFIG_DIR` gives the stderr line, then the ready line.
   - `bare_commands_use_the_context_and_stop_on_a_stale_one`: bare `inspect`, `pending`, `tools`
     and `call management.list.v1` succeed with decision 3's line; `--store` of a second serving
     store reaches it with no context line; each stop of decision 3, one after `serve` has ended
     among them, exits 2 with its line and leaves the store's `state.json` byte for byte.
   - `init_writes_the_flow_skeleton_unless_a_manifest_is_there`: decision 4's two lines byte for
     byte, with no context; each of MAN-001's three names left as it was; `init --store` run in an
     empty directory leaves it empty.
   - `install_keeps_the_archive_in_the_package_cache`: decision 5; a digest that does not match
     writes nothing; `conformance/fixtures/install-archives/missing-dependency.tar.gz`, which the
     engine refuses, is still kept.
   - `install_changes_no_manifest`: decision 6. Bare and with `--store`, run in an empty
     directory, in one holding decision 4's skeleton, and in one whose manifest pins the
     archive's address at another version: no file there is created, changed or removed, and
     standard output is the engine's one result line.

   Each existing helper or test body that starts `serve` or runs `install` through the binary
   gives the child a `CONDUIT_CONFIG_DIR` in its temporary directory, unless it already clears
   the environment with a temporary `HOME`. Change an existing assertion only where it reads
   `install`'s stderr.
8. **The documents.** `help.rs`: `[--store DIR]` where the context serves; the footer gives
   decisions 1 and 3, with status 2; `init`'s row both roles; `install`'s the cache, and that it
   changes no manifest (no text in `help.rs` or `engine/README.md` says `install` writes a pin).
   `engine/README.md` (its shell section and "Change a serving project"): the same.
   K3's README: `O8: Decided.`, with decisions 1 to 3 in a few lines; O7's and the Commands
   paragraph's sentences that leave the place, the context and the version to a later order
   (241 names this one) point at O8. Change nothing else in K3.

## What to read

- `spec/README.md`, "An author of documents" and "What the short form's meaning covers" (as
  order 240 leaves it); MAN-001 in `spec/promises/manifest.yaml`; PKG-004 and PKG-027 in
  `spec/promises/packages.yaml`.
- [K3](../implementation/plan-and-apply/README.md): its Commands section, O7, O8 and the S1
  schedule.
- In `engine/crates/shell/`: `src/main.rs` (arguments, `--store`, the ready line), `help.rs`,
  `author.rs` (`run`'s serving check, `install`), `boundary.rs` (`install_arguments`), `lib.rs`
  (`Config::for_store`), `store.rs` (`atomic_write`), and the test helpers that run the binary.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor cc3605a3 HEAD; echo "main $?"
awk '/^## DEC-008/,/^## DEC-009/' docs/decisions/log.md | grep -c '^Status: done'
grep -c '479ef470' docs/implementation/plan-and-apply/cases/01-t1.yaml
grep -c '^O8: Decided' docs/implementation/plan-and-apply/README.md
cat engine/crates/shell/src/*.rs | grep -c 'CONDUIT_CONFIG_DIR'
test -e engine/crates/shell/tests/context.rs; echo "context $?"
test -e "$HOME/.config/conduit"; echo "home $?"
test -n "${CONDUIT_CONFIG_DIR+x}"; echo "variable $?"
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null); echo "engine $?"
git status --short | wc -l | tr -d ' '
```

They print `main 0`; `1`, once order 240 has set DEC-008 to done (`0` on `main` at `cc3605a3`);
`0`, once order 241 has moved K3's cases to today's agents fixture (`13` at `cc3605a3`); then `0`,
`0`, `context 1`, `home 1`, `variable 1`, `engine 0` and `0`. `home 1` and `variable 1`: a test
that forgets decision 7's variable would write your home; the last check looks again.

## Scope

Change `engine/crates/shell/src/`, `engine/crates/shell/tests/` (the new file, and, for decision 7
alone, the existing places that start `serve` or run `install` through the binary),
`engine/README.md` and `docs/implementation/plan-and-apply/README.md` for decision 8, and
`engine/STANDING.md` only by `promise_standing.py --write` if its check asks.
Change nothing else: not `spec/`, `conformance/`, `packages/`, `experiments/`, the loader, the
core, a K3 case, any `Cargo.toml` or `Cargo.lock`, `IN_FORCE.md` or `ROADMAP.md`. Put anything else that needs changing in your report.

## Done when

Write decision 7's tests first and run them on the unchanged code: report each failing line. Then
change the code until all pass. Each line in the first block prints otherwise on `main` at
`cc3605a3` with 240 and 241 merged ("fails today"), and passes after.

```sh
t() { (cd engine && cargo test --release -p conduit-shell --test context "$1" -- --exact 2>&1 | grep -c '^test result: ok\. 1 passed'); }
t serve_records_the_engine_context  # 1; fails today (0: no such target)
t bare_commands_use_the_context_and_stop_on_a_stale_one  # 1; fails today (0)
t init_writes_the_flow_skeleton_unless_a_manifest_is_there  # 1; fails today (0)
t install_keeps_the_archive_in_the_package_cache  # 1; fails today (0)
t install_changes_no_manifest  # 1; fails today (0)
grep -c '^O8: Decided' docs/implementation/plan-and-apply/README.md  # 1; fails today (0)
grep -c 'CONDUIT_CONFIG_DIR' engine/crates/shell/src/help.rs  # 1 or more; fails today (0)
grep -c 'CONDUIT_CONFIG_DIR' engine/README.md  # 1 or more; fails today (0)
```

These pass, as on `main`; run the whole suite once, last, then look at your home again:

```sh
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy -p conduit-shell --all-targets -- -D warnings)
(cd engine && cargo test --release -p conduit-shell --test cli help_describes_every_command_without_store_access)  # 1 passed
git add -N engine  # so the checks see the new file
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_docs.py  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_private_paths.py  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/promise_standing.py --check  # Errors: 0
git diff --check  # prints nothing
git diff --name-only HEAD | grep -vE '^engine/crates/shell/(src|tests)/|^engine/(README|STANDING)\.md$|^docs/implementation/plan-and-apply/README\.md$' | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --release)  # the whole suite; once, last
test -e "$HOME/.config/conduit"; echo "home $?"  # home 1: no test wrote your context or cache
git reset -q  # stage nothing
```

Your report gives, per decision, what changed; each new test's failing line, then its passing
line; each existing helper, test body and assertion you changed; every choice you made
yourself, under `own_choices`; and every check's last line. Stage nothing.

## What follows

Named here and not in this order:

- **The dogfood's context.** Before the dogfood engine runs a build with this order, the
  coordinator gives its `serve` a `CONDUIT_CONFIG_DIR` of its own, and the S1 rerun's runbook
  gives each lab one, so that no lab's bare command reaches the dogfood's store; the dogfood keeps
  `--store`.
- **Pins in the author's manifest.** They come with the resolver design the owner asked to be
  researched (answer 1b): which release a pin names, how a dependency's version is chosen, and
  when a pin moves. Until it is built, the author writes each pin, digest included, by hand.
  This order leaves unbuilt, and does not change, `spec/README.md`'s sentence that `install`
  writes pins (ruling 53 point 5), and ruling 63 point 1 counts install-written pins as tool
  work; how both read until the resolver lands, K3's "After `conduit install` moves a package pin"
  and its T1, T2 and T5 counts, and the matching changes to orders 246 and 250 and the S1 rerun's
  counts, are the coordinator's to settle before those orders run.
- 245 reads the cache through decision 1's function; 245 and 247 compare `version`. With `--store`
  no context is read, and MCP's `serverInfo.version` lacks the commit: that case is theirs.
- 246 documents the context, `init` and `install` for the fresh author, `install` writing no pin.
- The gate's engine paths omit `docs/`: a change to K3's cases alone does not rerun 241's replay.
- Pruning the cache (as `go clean -modcache` does): later.

## Record

Run on 2026-11-21 through Conduit itself, by the dogfood's own Agents, in Pacific time.

**Worker-2's first turn** (20:57 to 21:12) built the per-user directory, the engine context
that `conduit serve` records, bare serving commands' use of it, `init`'s skeleton and the
package cache; its five new tests failed first and pass. It stopped before its last checks on
two gaps in the repository's own checks: `tools/set_names.py` took `CONDUIT_CONFIG_DIR` for an
unknown run identity variable, and `tools/check_docs.py` read the help's `packages/HEX` as a
repository path. The coordinator widened the scope by exactly one exception in
`tools/set_names.py`, in its own exact set with a near miss still rejected, and had the help
reworded, with no change to the docs check (open to the owner's strike).

**Worker-2's second turn** (21:12 to 21:42) made those two changes and finished the order. The
whole engine suite passed once, and the default per-user directory was never created.

**Two blind reads at once** (21:43 to 21:46), the owner's answer for A77's window:
`turn-verifier-2.turn.v1` found one must and one should; the Claude reader passed it with one
should and one nit, and found neither of the verifier's. One fix turn (21:47 to 21:54), the
must reproduced by a failing regression first:
- a bare command accepted a context after its store lost `engine-url.json`, since the store's
  default URL stood in; context selection now requires the persisted URL;
- a path holding a line break split a one-line diagnostic; such paths are now escaped;
- the quiet root probe replaced every failure with fixed words; it now says whether the engine
  rejected the store's token, answered with another content type or answered with an error;
- the engine README keeps its advice to parse the ready line as JSON.

Eight context tests pass. Install still writes no pin: order 263 builds DEC-048's exact pins
and closure walk. The gate's run on the squash is the coordinator's, at the merge.
