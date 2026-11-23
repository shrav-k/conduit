# Work order 291: the runtime provider kit and ssh-runtime serve `run.v2`, checking the archive's size and SHA-256 on the node before each unpacking (DEC-049, DEC-080)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; `vectors` is
empty. This order needs no network: it reaches no host, starts no `sshd` and fetches no crate.
Run cargo offline; a crate missing from the cache is a step outside the scope (the stop rule):
fetch nothing. Run one cargo command at a time, and not `tools/gate.py`. Never print a whole
large file, diff or log: use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never read or
print a key, a token or any credential, even one a test made. Run no `conduit` outside a test, no
`ssh`, `codex` or `claude`, and sign in nowhere. Prefix each `python3` with
`PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute local path, user or
machine name, or private repository name.

## What and why

**The owner's answers**: [DEC-049](../decisions/log.md#dec-049-2026-11-21-per-platform-programs-in-one-archive),
per-platform programs in one archive, and
[DEC-080](../decisions/log.md#dec-080-2026-11-22-the-per-node-chain-is-finished-now), "Finish the
chain now". Order 274 wrote `run.v2` and PRV-019's check; order 285, in build, has the engine
send `run.v2`. This order has a runtime provider answer it.

**Today**, on `main` at `52a29f1a`:

1. The kit (`packages/runtime-provider-kit`) offers exactly `run.v1`, `stop.v1` and
   `inspect-run.v1` (`schemas::OPERATIONS`). `src/schemas.json` holds no `run.v2` and no
   `digest.v1`. `RunInput` has no `program` or `archive`. `Declaration::supports` refuses a
   `command` whose program holds `/` and does not begin with one.
2. ssh-runtime (`packages/provider-ssh-runtime`) is 0.1.0. Its Type `ssh-runtime.v1` has
   `requires: []` and maps the three; its `provider` item implements them. `stage` refuses a
   relative program and resolves an absolute or bare one on the node. Nothing carries an archive.
3. `packages/provider-local-process` builds its `Declaration` from the four fields.

**No case.** No promise or spec file changes, so CONTRIBUTING's rule 8 asks no case, no
`spec/CHANGES.md` line is due, and no model-account row changes.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 3, a
shared thing is owned once: one archive and one digest, and each node keeps one copy per digest,
shared by its runs and proved again before each use; 6, nothing repeats an effect implicitly: a
failed check runs nothing and retries nothing, and bytes are carried only for a run the engine
asked for; 4, nothing assumes a vendor's agent: any package's item that states `platforms`. None
at risk: `run.v1`'s answers are as today.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)).
Yes, by existing practice: containerd keeps each blob in its content store by digest, checks a
descriptor's size and digest, and unpacks layers into a snapshot of each container's own; VS
Code's Remote-SSH (`remote.SSH.localServerDownload`) downloads its server on the local machine,
copies it to the host over SSH and reuses that copy; a CSI orchestrator calls only the RPCs a
plugin advertises. Not taken (the coordinator's design notes): a check on arrival only, since an
unconfined Agent can alter a kept copy (ruling 70); an unpacked tree kept per digest and run in
place, which no digest covers; the node fetching the archive, since only the engine's machine
holds the copy (PRV-002).

**When it runs.** On `main` from `52a29f1a` (#563). It needs no engine order first: its tests
use the fakes, and order 285 sends the call. Through Conduit: one Codex worker turn, both blind
reads at once, one fix turn, the gate (DEC-046, DEC-050, DEC-033). By ruling 89, each decision
says what must hold; how is yours. The words, the messages, `packaged_programs`, the kept copies'
place, the version and the test names are the coordinator's, open to the owner's strike.

**Shared files.** Order 282 changes ssh-runtime's and provider-local-process's
`src/bin/package.rs`; this order leaves those files alone, and the later to merge rebases. 285
changes no package.

**Stop rule** (the coordinator's D1). Beyond the start check, stop, change nothing further, and
report only for: a contradiction between promises (give both); a step that needs a change outside
the scope (give it); or a file outside Scope. Where this order's wording contradicts a promise it
leaves unchanged, the promise governs: follow it and list it under `own_choices`. A choice no
decision governs is yours: list it under `own_choices`.

## The decisions

### The kit

1. **The schema copy** (the kit's README, "Schema copies and checks"). `run.v2` joins
   `schemas::OPERATIONS`: `src/schemas.json` gains its two schemas and `digest.v1`, made with the
   drift test's own conversion, as order 274's Record did, so the drift test compares `run.v2`
   too. In the README, "The eight transitively reached" becomes "The nine transitively reached".
2. **`packaged_programs`** (PRV-001). `Declaration` gains this boolean, which each runtime states.
   Where it is true, the server lists and serves `run.v2` beside the three. Where it is false,
   `tools/list` gives the three, and a `run.v2` call fails as `unknown runtime operation`, as an
   unknown name does today, before any backend effect. ssh-runtime states true;
   provider-local-process states false, and nothing else of that package changes.
3. **A `run.v2` call** (`run.v2`, PRV-019). Every rule the kit applies to a `run.v1` call holds
   for it: the drain first, the repeated-run, owed-stop and admission policies, the gate, the
   output schema and the reasons. In place of `supports`' refusal of a relative program, two
   checks come before any backend effect, each failing the call with its fixed text:
   - `item` is a `command` item whose program is a relative path (MAN-010) and that states
     `platforms`; else `run.v2 item must be a command item with a relative program that states
     platforms`;
   - `program` does not begin with `/` and has no `..` part; else `run.v2 program must be a
     relative path inside the archive`.

   Staging receives `program` and `archive` in `RunInput`. For a `run.v1` call the kit gives
   staging neither, whatever its arguments hold, and answers as today. The ledger and the records
   hold neither.

### ssh-runtime

4. **The contract** (PRV-001, PRV-019, TPL-004, PKG-025). `ssh-runtime.v1` lists
   `github.com/shrav-k/conduit/core#run.v2` under `requires`, in flow style as `offers` is, and
   maps it to `'#provider'`; the `provider` item implements it beside the three. The Type's
   description is the provider's contract, and the README's "Contract" section repeats it: both
   limit the relative-program refusal to `run.v1` and state `run.v2`, that is how the bytes
   travel, where kept copies live and that the operator removes them, the check and its tools,
   and that the check does not stop another run under the shared `conduit-run` user from
   altering an unpacked program between its unpacking and its start (PRV-025's isolation is
   unmet). The package becomes 0.2.0 in `conduit.yaml`, `Cargo.toml`, its own `Cargo.lock` entry
   and the README's opening sentence (line 3), since a store with 0.1.0 installed refuses other
   bytes at that version (PKG-025). No dependency is added.
5. **Carrying and keeping** (PRV-019, PRV-002). The provider reads the archive from `urls[0]`, a
   `file` URL (RFC 8089) of a path on its own machine, the engine's: `file:///<path>` or
   `file://localhost/<path>`, the path percent-decoded. Each node keeps one copy per digest, at
   `<base>/<part>/archives/<hex>`, `<hex>` the digest's 64 digits. The provider carries the bytes
   only where no copy is kept there, over that machine's SSH connection: a call's standard input
   is the raw bytes, never an argument and not a word of a script (`codec::write_file` makes one
   four times their size). That call writes a temporary file under `archives/` and renames it to
   `<hex>`, so a cut transfer leaves nothing under the digest's name; where the fake remote runs
   that call otherwise, `tests/support/` changes with it. Sweeps and stops leave `archives/`
   alone. Any other URL, or a copy it cannot read, ends staging failed: `archive URL is not a
   file URL on the provider's machine` or `the engine's copy of the archive cannot be read`.
6. **The check before each unpacking** (PRV-019, as 274 amended it). On the node, by its own tools
   on the run's PATH, the byte length of the file about to be unpacked is compared with `size`,
   and only where they match is its SHA-256 compared with `digest`. The bytes checked are the
   bytes unpacked: a file of the run's own, not a second read of the kept copy. A mismatch runs
   nothing, removes the kept copy so that a later run carries the engine's afresh, and ends
   staging failed: `archive on machine <hostname> has <found> bytes; the call gave <size>` or
   `archive on machine <hostname> has digest <found>; the call gave <digest>`, both digests
   written `sha256:<hex>`. A found value is written only where it parses as digits or as 64
   lowercase hexadecimal digits, never as a tool's output passed through; where it does not,
   nothing runs and staging ends failed with `archive check on machine <hostname> gave no
   readable length` (or `digest`). A node that lacks a tool the check or the unpacking runs ends
   staging failed with `machine <hostname> lacks <tool> on the run PATH`, before anything is
   carried.
7. **The unpacking and the run** (PRV-019, MAN-013). For `run.v2` alone, ssh-runtime's `stage`
   applies decision 3's checks in place of `supports`' relative-program refusal; `run.v1` keeps
   today's. Where size and digest match, the archive is unpacked into a new, empty `package/`
   directory that staging makes in the run's generation directory, apart from `identity/` and
   `environment`; stops and `ExecStopPost` already remove it. `program` there must be a regular
   file its owner may execute, else staging ends failed with `archive on machine <hostname>
   holds no executable <program>`. The unit runs that file's absolute path in place of
   `command[0]`, the item's other strings its arguments. The rest of staging and start is as for
   `run.v1`.

### Tests and documents

8. **Tests**, with the kit's fake backend and ssh-runtime's fake remote (`tests/support/`), which
   runs every `/bin/sh` call as `/bin/sh -s` on the test host: no `sshd`, no other machine. Write
   them first, run them on the unchanged code, and report each failing line. The module is yours.
   - Kit, `run_v2_reaches_staging_with_the_program_and_the_archive`: a fake that declares
     `packaged_programs` lists four tools; a `run.v2` call, its item `command: [bin/server,
     --flag]` with `platforms: [linux/arm64]`, its `program` `platforms/linux/arm64/bin/server`
     and an `archive` descriptor, reaches staging with both and succeeds with the gate's URL; the
     ledger holds neither.
   - Kit, `run_v2_is_refused_unless_declared_and_well_formed`: with `packaged_programs` false,
     three tools and `unknown runtime operation`; with it true, the `program`
     `platforms/../bin/server`, an absolute `program`, an item without `platforms` and an item
     with an absolute program each fail with decision 3's text; none reaches staging; `run.v1` of
     the relative item fails as today; `run.v1` of an absolute item, with `program` and `archive`
     keys added, reaches staging with neither.
   - ssh-runtime, `run_v2_carries_the_archive_once_and_runs_the_pair_program`: an archive built
     in the test (the `tar` and `flate2` crates) holding an executable
     `platforms/linux/arm64/bin/server`, named by a `file` URL whose path holds a space. Staging
     two instances carries the bytes in exactly one call, keeps one copy under the digest's hex,
     and unpacks into each run's own `package/`; each staged command is that directory's program
     with `--flag`; a start through the fake succeeds.
   - ssh-runtime, `run_v2_mismatch_runs_nothing_and_names_both_values`: a `size` one more than the
     file's fails staging with decision 6's size text naming both lengths, nothing unpacked. Then,
     with the kept copy altered at the same length, a `run.v2` call through the kit's server, as
     `run_answers_carry_backend_reasons_without_values` makes one, answers failed with `staging
     failed: ` and the digest text naming both digests; nothing is unpacked, no unit starts and
     the kept copy is gone.
   - ssh-runtime, `run_v2_names_a_missing_tool`: a run PATH without `sha256sum` fails staging with
     decision 6's tool text, and nothing is carried.

   The scripts stay POSIX, as the sweep's comment says: the fake runs them on macOS too, where
   `sha256sum` is only in `/sbin` and `wc -c` pads its count; a test's run PATH may name `/sbin`.
   An existing test changes only where it asserts what decisions 1 to 7 change.
9. **The READMEs.** The kit's: `run.v2`, `packaged_programs` and decision 3. ssh-runtime's: its
   "Contract" section as decision 4 says, and the check's messages.

## What to read

PRV-001, PRV-002, PRV-007, PRV-019, MAN-010, MAN-013, PKG-025 and TPL-004; `run.v1` and
`run.v2`. The kit's `src/backend.rs`, `src/server.rs` (`execute`, `list_tools`, `call_tool`),
`src/schemas.rs`, `tests/schema_drift.rs` and `tests/support/`; ssh-runtime's `src/backend.rs`
(`stage`, `start`, `sweep`), `src/codec.rs`, `contracts.yaml`, `conduit.yaml` and
`tests/support/mod.rs`, by part with `sed -n`.

## Before you change anything

From the repository root; if a line prints otherwise, change nothing and report it:

```sh
git merge-base --is-ancestor 52a29f1a HEAD; echo "main $?"
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
j spec/promises/providers.yaml 'It first checks that the byte length is the `size` of `archive`.'
grep -c '^  name: run.v2$' spec/core/operations.yaml
grep -cF 'pub const OPERATIONS: [&str; 3] = ["run.v1", "stop.v1", "inspect-run.v1"];' packages/runtime-provider-kit/src/schemas.rs
j packages/runtime-provider-kit/README.md 'The eight transitively reached Schema contracts'
grep -c '^  requires: \[\]$' packages/provider-ssh-runtime/contracts.yaml
grep -c '^version: 0.1.0$' packages/provider-ssh-runtime/conduit.yaml
git grep -l -e 'run\.v2' -e 'packaged_programs' -- packages | wc -l | tr -d ' '
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
for d in packages/runtime-provider-kit packages/provider-ssh-runtime packages/provider-local-process engine; do (cd "$d" && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "offline $? $d"; done
git status --short | wc -l | tr -d ' '
```

They print `main 0`, `1` six times, `0`, four `offline 0` lines with their directory, and `0`.

## Scope

Change the kit's `src/`, `tests/` and `README.md`; ssh-runtime's `src/`, `tests/`, `README.md`,
`conduit.yaml`, `contracts.yaml`, `Cargo.toml` (its version alone) and `Cargo.lock` (its own
entry alone); and `packages/provider-local-process/src/backend.rs` for decision 2's one field.
Change nothing else: not `spec/`, `engine/`, `conformance/`, `docs/`, `tools/`, another package,
or another `Cargo.toml` or `Cargo.lock`. Report what else needs changing.

## Done when

Each line of the first group prints otherwise today and passes after. The second group are
guards, which pass today and must still pass; run its last six lines, the package suites and the
engine's three, once each, last. Engine tests: the `local_process` regression, the core's tests
and the whole vectors test; never the whole engine suite.

```sh
ok='^test result: ok\. 1 passed'
for t in run_v2_reaches_staging_with_the_program_and_the_archive run_v2_is_refused_unless_declared_and_well_formed; do (cd packages/runtime-provider-kit && cargo test --offline --release "$t" 2>&1 | grep -c "$ok"); done  # 1 twice; today 0
for t in run_v2_carries_the_archive_once_and_runs_the_pair_program run_v2_mismatch_runs_nothing_and_names_both_values run_v2_names_a_missing_tool; do (cd packages/provider-ssh-runtime && cargo test --offline --release "$t" 2>&1 | grep -c "$ok"); done  # 1 three times; today 0
PYTHONDONTWRITEBYTECODE=1 python3 -c "import json;d=json.load(open('packages/runtime-provider-kit/src/schemas.json'));print(sorted(d['operations']),'digest.v1' in d['definitions'])"  # ['inspect-run.v1', 'run.v1', 'run.v2', 'stop.v1'] True; today no run.v2, False
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
j packages/runtime-provider-kit/README.md 'The nine transitively reached Schema contracts'  # 1; today 0
grep -c 'packaged_programs' packages/runtime-provider-kit/README.md  # 1 or more; today 0
grep -cE '^  requires: \[.*core#run\.v2.*\]$|^    github\.com/shrav-k/conduit/core#run\.v2: .#provider.$' packages/provider-ssh-runtime/contracts.yaml  # 2; today 0
grep -c 'archives/' packages/provider-ssh-runtime/contracts.yaml  # 1 or more; today 0
grep -c 'archives/' packages/provider-ssh-runtime/README.md  # 1 or more; today 0
grep -c '^    - github.com/shrav-k/conduit/core#run.v2$' packages/provider-ssh-runtime/conduit.yaml  # 1; today 0
grep -c '^version: 0.2.0$' packages/provider-ssh-runtime/conduit.yaml  # 1; today 0
grep -c '^version = "0.2.0"$' packages/provider-ssh-runtime/Cargo.toml  # 1; today 0
j packages/provider-ssh-runtime/README.md 'provider-ssh-runtime` 0.2.0'  # 1; today 0
grep -c 'packaged_programs: false' packages/provider-local-process/src/backend.rs  # 1; today 0
```

```sh
for d in runtime-provider-kit provider-ssh-runtime provider-local-process; do (cd packages/$d && cargo fmt --check && cargo clippy --offline --all-targets -- -D warnings >/dev/null 2>&1); echo "lint $? $d"; done  # lint 0 three times
export PYTHONDONTWRITEBYTECODE=1
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
python3 tools/check_docs.py | tail -1  # Errors: 0
python3 tools/check_private_paths.py | tail -1  # Errors: 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^packages/(runtime-provider-kit/(src/.+|tests/.+|README\.md)|provider-ssh-runtime/(src/.+|tests/.+|README\.md|conduit\.yaml|contracts\.yaml|Cargo\.(toml|lock))|provider-local-process/src/backend\.rs)$' | wc -l | tr -d ' '  # 0
(cd packages/runtime-provider-kit && cargo test --offline --release)  # every test passes, the drift test among them
(cd packages/provider-ssh-runtime && cargo test --offline --release)  # every test passes
(cd packages/provider-local-process && cargo test --offline --release)  # every test passes
(cd engine && cargo test --offline --release -p conduit-shell --test local_process)  # passes, local-process built on the changed kit
(cd engine && cargo test --offline --release -p conduit-core)  # every test passes
(cd engine && CONDUIT_VECTOR_THREADS=4 cargo test --offline --release -p conduit-conformance --test vectors)  # every vector passes
```

Report, per decision, what changed and where; the kept copy's place and how the bytes travel;
each new test's failing line on the unchanged code, then its passing line; each existing test you
changed, and why; and every check's last line. Stage nothing.

## What follows

- **Order 285**, in build: the engine sends `run.v2`; with both merged, ssh-runtime 0.2.0 runs a
  package's item that states `platforms` on a node.
- **The packages'** (DEC-051): the two Agent packages for `darwin/arm64`, `linux/amd64` and
  `linux/arm64`, the Linux ones built in Docker.
- **R-3**: ssh-runtime's first run against a real `sshd` on a node (the coordinator's design
  notes).
- **A short engine order**: PRV-001's rejection and `run.v2` among PKG-020's and PKG-030's
  operations, as 285's "What follows" names it.
- Found while writing: nothing removes a node's kept copies; a retention rule, as containerd
  collects unreferenced blobs, is a later order.

## Record

Run on 2026-11-22 through Conduit itself, by the dogfood's own Agents, in Pacific time, on
`main`.

**Worker-1's turn** (14:42 to 15:05) built DEC-080's provider side: the runtime provider kit and
ssh-runtime 0.2.0 serve `run.v2`, checking the kept archive's size and SHA-256 on the node before
unpacking it to a per-run place and running the selected program; local-process keeps its three
tools. The five new tests failed on the unchanged code and pass, with the three package suites.

**Two blind reads** (the Claude reader 15:07 to 15:12, the Codex verifier 15:16 to 15:21). The
verifier's three musts: the size check, the digest check and the unpacking could read different
bytes; a failed removal left a known-bad archive eligible for reuse; the kit refused a valid size
written as an integral JSON decimal. The reader's must: an unpacking failure named only the
machine; with two shoulds and two nits on the node-side diagnostics, the recovery after a
mismatch, the node's tools and the kit's quoted refusals. One fix turn (15:21 to 15:35) applied
all eight, each must's regression failing first: 97 kit, 70 ssh-runtime and 22 local-process
tests pass. The gate's run on the squash is the coordinator's, at the merge.
