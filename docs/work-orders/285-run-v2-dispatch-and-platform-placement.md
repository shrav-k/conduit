# Work order 285: the engine asks `run.v2` for a package's item that states `platforms`, with the node's program and the kept archive, and places it only on a listed pair (DEC-049, the run side)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; list under
`vectors` the two cases you add, with the promises each tests. This order needs no network: it
reaches no host and fetches no crate. Run cargo offline; a crate missing from the cache is a step
outside the scope (the stop rule): fetch nothing. Run one cargo command at a time, and not
`tools/gate.py`. Never print a whole large file, case, diff or log: use `head`, `tail`, `grep`,
`sed -n` or a count. Never run `env`, `printenv`, `set` or `export -p`, never print an environment
variable's value, and never read or print a key, a token or any credential, even one a test made.
Run no `conduit` outside a test, no `npm`, `node`, `codex` or `claude`, and sign in nowhere. Prefix
each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute local
path, user or machine name, or private repository name.

## What and why

**The owner's answers**: [DEC-049](../decisions/log.md#dec-049-2026-11-21-per-platform-programs-in-one-archive),
per-platform programs in one archive, and
[DEC-080](../decisions/log.md#dec-080-2026-11-22-the-per-node-chain-is-finished-now), "Finish the
chain now". Order 274 wrote the spec; order 278 built the install side. This order is the run
side.

**Today**, read on `main` at `c4fa2197` with 278 stacked:

1. Placement (`registration.rs`, the loop that ends `node does not match {key}`) checks the
   selector label by label and never reads `platforms`: an item listing `darwin/arm64` and
   `linux/arm64` is placed on a `linux/amd64` node.
2. On `stopped` (`lifecycle.rs`, `Report::InspectRun`), an instance that names a `runtime` and whose
   item is in `relative_programs`, a package's that states `platforms` among them, has its part
   failed with no diagnostic (`refuse-relative-program-provider-run`); any other goes to
   `secrets::emit_run`, which asks `run.v1`. A retirement's run (`retirement.rs`) asks `run.v1` with
   no such check. The shell's `provider_later`, its identity (`lib.rs`), the bridge and the reports
   in `lifecycle.rs` and `retirement.rs` know `run.v1` alone.
3. The resolved item's relative program is expanded against the install `directory`
   (`expand_manifest`), a path on the engine's machine.
4. The engine's own runner (`runner.rs`, the installed package's program) runs `command[0]` from
   the package's files, never `platforms/<os>/<arch>/`.
5. 278 keeps each archive in the store as `packages/HEX`, and the shell's `Store` knows which
   digests it keeps; the core's State holds the digest alone.
6. The conformance runner's record checks (its `records.rs`) name `run.v1`, not `run.v2`.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 3, a
shared thing is owned once: the call carries the one kept copy, by its digest, and the node's
program is chosen from it; 7, everything is attributable: each `run.v2` call is a recorded
provider call naming its instance; 6, nothing repeats an effect implicitly: `run.v2` is asked only
where the provider reports the run `stopped`, as `run.v1` is. None at risk: replies change only by
decisions 1, 2, 4 and 6.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)).
Yes, by existing practice: a kubelet admits no Pod whose `kubernetes.io/os` or arch its Node does
not match, and an OCI image index gives the manifest whose `platform` is the node's; a runtime is
handed an OCI descriptor (`digest`, `size`, `urls`) and verifies the blob; a CSI orchestrator
calls only the RPCs a plugin advertises. Not taken (the coordinator's design notes): `run.v1` with
the engine's absolute path to the pair's program; the engine copying the archive to the node (the
provider's, PRV-019); the length kept in `state.json`, beside the copy that gives it.

**When it runs.** On `main` from `c4fa2197` (#559), with order 278 stacked until it merges. Through
Conduit: one Codex worker turn, both blind reads at once, one fix turn, the gate (DEC-046, DEC-050,
DEC-033). By ruling 89, each decision says what must hold; how is yours. The words, the messages,
the split of decisions 3 and 4 between the core and the shell (which holds the copy), and the test,
fixture and case names are the coordinator's, open to the owner's strike.

**Shared files.** 278 changes the shell's `store.rs` and `lib.rs`, `make_archives.rs`, the
install fixtures and `engine/STANDING.md`; 272, 273 and 275 change core `requests.rs`,
`registration.rs` and `lifecycle.rs` and both `index.yaml` files. The later to merge rebases and
reruns `promise_standing.py --write`.

**Stop rule** (the coordinator's D1). Beyond the start check, stop, change nothing further, and
report only for: a contradiction between promises (give both); a step that needs an engine change
outside the scope (give it); or a file outside Scope. Where this order's wording contradicts a
promise it leaves unchanged, the promise governs: follow it and list it under `own_choices`. A
choice no decision governs is yours: list it under `own_choices`.

## The decisions

### Placement

1. **The pairs** (PLC-007, PKG-019, MAN-010). Where an item that is run and is selected for an
   Agent or a Resource states `platforms`, it is not instantiated unless the node's fixed
   `kubernetes.io/os` and `kubernetes.io/arch` (ENR-010), as `<os>/<arch>`, is one of its pairs.
   A node that lacks either label matches none; an os or an arch alone matches nothing. It is
   checked wherever the selector is, after it, before any effect: one error, `ruleId` `PLC-007`,
   its text naming the item, the node, the node's pair (`none` where a label is absent) and the
   listed pairs, as `server lists platforms darwin/arm64, linux/arm64; node local is
   linux/amd64`. An item that states no `platforms` is placed as today.

### The run

2. **Which call** (PRV-007, PRV-001, TPL-004). Where the runtime provider of an instance that
   names a `runtime` reports the run `stopped`:
   - for an installed package's item that states `platforms`, the engine asks `run.v2`, and never
     `run.v1`, where the provider's Type lists the core's `run.v2` under `requires`, TPL-004
     giving the provider its implementation; otherwise it asks nothing;
   - for any other item whose program the manifest gives as a relative path, it asks nothing, as
     a setup does today;
   - for every other item it asks `run.v1`, as today, whatever the provider implements.

   Where it asks nothing, the part ends failed with one diagnostic, `ruleId` `PRV-007`, naming
   the item and the provider, and then exactly one of `the provider does not implement run.v2;
   its type does not list it under requires` or `only the engine's own runner runs a relative
   program that is not a package's item stating platforms`. A retirement's run for its retire
   hook (TPL-018) is chosen the same way.
3. **Its inputs** (PRV-007, `run.v2`). `run.v1`'s inputs as today, except: `item` is the item as
   its package's manifest writes it, its program the relative path, not expanded against the
   install `directory`; `program` is `platforms/<os>/<arch>/` followed by that path, the pair the
   node's; and `archive` gives the package's `digest` (PKG-004). The core adds no `size` or `urls`.
4. **The kept copy** (PRV-007, DL7). Before it sends a `run.v2` call, the shell completes `archive`
   from the copy that 278 keeps as `packages/HEX` (hashed as the store opens): `size`, that file's
   length in bytes, and `urls`, one item, the `file` URL (RFC 8089) of its absolute path. Where the
   store keeps no copy for the package, as for one installed before 278, its members in
   `state.json`, or the file is absent or not a regular file, the shell sends nothing, and the part
   ends failed with one `PRV-007` diagnostic naming the item, the provider, the package's address
   and version, the copy's path, and `no kept archive`, `absent` or the kind found. Nothing repairs
   or writes a copy here.
5. **The call, its result and its records** (PRV-005, PRV-028, IFC-017, REC-002, PRV-024). A
   `run.v2` call is a provider call as `run.v1`'s is: its identity is materialized; the `url` it
   returns is proved and bridged before anything is sent there; a result with no `url` ends the
   part unknown; a cancelled setup sends nothing; a missing executor reports as for `run.v1`.
   Each report, record and diagnostic names the operation that was asked. The call's record
   names the instance in the CloudEvent's `subject` and holds no `identity` or `secrets`; where
   it holds the inputs, its `archive` holds the digest alone, no local path.
6. **The engine's own runner** (PRV-007, MAN-010). For a package's item that states `platforms`,
   the engine's own runner runs `platforms/<os>/<arch>/` followed by the program from the
   package's files, the pair `local`'s, with the item's arguments.

### Tests and cases

7. **Tests**, in `engine/crates/shell/` (the module is yours). Write them first, run them on the
   unchanged code, and report each failing line. Take the scripted provider from
   `tests/provider_calls.rs` and the install from 278's `kept_archives.rs`.
   - `run_v2_sends_the_pair_program_and_the_kept_archive`: an archive built in the test, its one
     item `command: [bin/server]` with `platforms` the host's pair (`local_machine`'s labels) and
     one other, installed; a Resource of a Type it implements, whose runtime is a scripted provider
     whose Type lists `run.v2` under `requires`. Its setup sends `inspect-run.v1`, then `run.v2` and
     no `run.v1`, with `program` `platforms/<host pair>/bin/server`, `item.command[0]` `bin/server`,
     `archive.digest` the package's digest, `size` the archive's length, `urls[0]` the copy's `file`
     URL, and `identity` as for `run.v1`; the returned `url` is proved and the setup succeeds; the
     call's record has `operation` the core's `run.v2` and `subject` the instance. Today the part
     fails and no run call is sent.
   - `run_v2_is_never_sent_to_a_run_v1_provider_or_without_a_kept_archive`: the same with a
     provider whose Type does not list `run.v2`: no run call, and decision 2's diagnostic; and
     with a store whose `state.json` holds the package's members, built as 278's guard test
     builds one: no run call, and decision 4's diagnostic; and, with that `run.v1` provider, a
     retirement whose retire hook (TPL-018) is the item: no run call, decision 2's diagnostic.
   - `own_runner_runs_the_program_for_the_nodes_pair`: with no runtime, the own runner starts
     `platforms/<host pair>/bin/server`, the archive holding no `bin/server`, and the setup
     succeeds. Today the program is not found.
   - An existing test changes only where it asserts what decisions 1 to 6 change.
8. **Two portable cases** (from `{"recipe":"empty"}`), each with its entry in its directory's
   `index.yaml`. The archive: 278's `platforms-complete.tar.gz` if a project can select its item,
   else one new fixture, `platforms-resource.tar.gz`, by `make_archives.rs`, exporting a Resource
   Type that `server` implements, every other fixture's bytes kept, its digest in `digests.json` and
   a line in the README. Take the install from `registration-agent-v2-packages` and the provider
   steps (`setup-report`) from `setup-review-relative-provider`.
   - `registration-placement-platforms`, in
     `conformance/language/registration/placement-platforms/`, promises `PLC-007` and `PKG-019`:
     first start with `local` at `linux/amd64`; the install; registering a Resource whose item
     states `darwin/arm64` and `linux/arm64`, placed on `local`, is rejected with decision 1's
     error, the case stating its whole text, and nothing is registered. Today it is accepted.
   - `setup-run-v2`, in `conformance/conventions/setup/run-v2/`, promises `PRV-007`, `PLC-007`,
     `PRV-005` and `REC-002`: first start with `local` at `linux/arm64`; the install; one
     registration of two runtime-provider Resources, `v2`, whose Type lists the core's `run.v2`
     under `requires`, and `v1`, whose Type does not, and two Resources of the package's Type on
     `local`, `a` with runtime `v2` and `b` with `v1`, accepted since the pair matches. `a`'s
     setup gives, after `inspect-run.v1` reports `stopped`, a `provider-call` intent for `run.v2`
     with decision 3's `program`, `item` and `archive.digest`; a `run` report with a `url` ends
     the part succeeded, and the call's record names `run.v2` and `a`. `b`'s gives no run intent,
     and its part fails with decision 2's diagnostic. Today `a`'s part fails with no diagnostic.

   If no portable route reaches a step, the case may be internal, as
   `setup-review-relative-provider` is: list it under `own_choices`. Declare both in
   `docs/work-orders/285-declared.yaml`, in the form of [256's](256-declared.yaml), and run
   `python3 tools/promise_standing.py --write`.
9. **The runner's record checks** (`records.rs`): each check it makes of a `run.v1` record, no
   `identity` or `secrets` (PRV-024) and a `subject` (REC-002), holds of a `run.v2` record too.

## What to read

The promises named, TPL-004, TPL-018, DL7 and ENR-010; `run.v1` and `run.v2`. The sources
"Today" names, `emit`, `run_inputs` and `record_content` among them, and 278's copy in
`store.rs`; the case `setup-review-relative-provider`, by step with `sed -n`.

## Before you change anything

From the repository root; if a line prints otherwise, change nothing and report it:

```sh
git merge-base --is-ancestor c4fa2197 HEAD; echo "main $?"
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
j spec/promises/providers.yaml 'only with `run.v2`, never with `run.v1`'
j spec/promises/placement.yaml 'are not one of its pairs.'
j spec/conventions/records.yaml 'A record of `run.v1`, `run.v2`, `stop.v1` or `inspect-run.v1` names'
grep -c 'From order 278,' engine/CONFORMANCE.md
grep -cF 'pub fn keep_archive(&self, digest: &str, bytes: &[u8])' engine/crates/shell/src/store.rs
grep -cF '"refuse-relative-program-provider-run",' engine/crates/core/src/lifecycle.rs
grep -cF 'if !operation.ends_with("#run.v1") {' engine/crates/shell/src/runner.rs
git grep -l 'run\.v2' -- engine conformance | wc -l | tr -d ' '
test -e conformance/language/registration/placement-platforms || test -e conformance/conventions/setup/run-v2; echo "cases $?"
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
git status --short | wc -l | tr -d ' '
```

They print `main 0`, `1` three times, `2`, `1` three times, `0`, `cases 1`, `engine 0` and `0`.
A `0` for `From order 278,` or `keep_archive` means 278 is not stacked.

## Scope

Change, for decisions 1 to 6, core `registration.rs`, `lifecycle.rs`, `secrets.rs`, `retirement.rs`,
`requests.rs` and `lib.rs`, and shell `runner.rs`, `lib.rs`, `store.rs` and `identity.rs`; for
decision 7, the shell's `src/tests.rs`, `src/*_tests.rs` and `tests/`, a new test file under
`tests/`; for decision 8, `make_archives.rs`, `conformance/fixtures/install-archives/`
(`platforms-resource.tar.gz`, `digests.json`, `README.md`), the two case directories,
`conformance/language/registration/index.yaml`, `conformance/conventions/setup/index.yaml` and
`docs/work-orders/285-declared.yaml`; for decision 9, `engine/crates/conformance/src/records.rs`;
and `engine/STANDING.md` only by the tool. Change nothing else: not `spec/`, another case, other
`docs/`, `packages/`, `tools/`, the loader, `Cargo.toml` or `Cargo.lock`. Report what else needs
changing.

## Done when

Each line of the first group prints otherwise today and passes after. The second group are guards,
which pass today and must still pass; run its last cargo line, the whole engine suite, once, last.

```sh
ok='^test result: ok\. 1 passed'
for t in run_v2_sends_the_pair_program_and_the_kept_archive run_v2_is_never_sent_to_a_run_v1_provider_or_without_a_kept_archive own_runner_runs_the_program_for_the_nodes_pair; do (cd engine && cargo test --offline --release -p conduit-shell "$t" 2>&1 | grep -c "$ok"); done  # 1 three times; today 0
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. registration-placement-platforms 2>&1 | tail -1)  # 1 cases passed; today no cases matched the requested prefix
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. setup-run-v2 2>&1 | tail -1)  # 1 cases passed; today the same
test -f docs/work-orders/285-declared.yaml; echo "declared $?"  # declared 0; today declared 1
```

```sh
n="conformance/language/registration/placement-platforms conformance/conventions/setup/run-v2 docs/work-orders/285-declared.yaml"
git add -N $n conformance/fixtures/install-archives  # so the checks see the new files; add any new test file too
for c in setup-review-relative-provider archives-platform-programs registration-placement-label-linux; do (cd engine && cargo run --offline --release -q -p conduit-conformance -- .. "$c" 2>&1 | tail -1); done  # 1 cases passed, three times
(cd engine && cargo test --offline --release -p conduit-shell --test provider_calls 2>&1 | grep -c '^test result: ok\.')  # 1
(cd engine && cargo test --offline --release -p conduit-shell --test kept_archives 2>&1 | grep -c '^test result: ok\.')  # 1
(cd engine && cargo run --offline --release -q -p conduit-conformance --example make_archives -- --check; echo "archives $?")  # archives 0
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-core -p conduit-shell -p conduit-conformance --all-targets -- -D warnings)
export PYTHONDONTWRITEBYTECODE=1
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
python3 tools/check_docs.py | tail -1  # Errors: 0
python3 tools/check_schemas.py | tail -1  # Errors: 0
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
python3 tools/check_private_paths.py | tail -1  # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/285-declared.yaml; echo "exit $?"  # 0 undeclared; 0 errors; then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(engine/crates/core/src/(registration|lifecycle|secrets|retirement|requests|lib)\.rs|engine/crates/shell/(src/(runner|lib|store|identity|([a-z]+_)?tests)\.rs|tests/.+)|engine/crates/conformance/(src/records\.rs|examples/make_archives\.rs)|conformance/fixtures/install-archives/(platforms-resource\.tar\.gz|digests\.json|README\.md)|conformance/language/registration/(index\.yaml|placement-platforms/.+)|conformance/conventions/setup/(index\.yaml|run-v2/.+)|docs/work-orders/285-declared\.yaml|engine/STANDING\.md)$' | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release)  # the whole engine suite, every vector among it
git reset -q -- $n conformance/fixtures/install-archives  # stage nothing
```

Report, per decision, what changed and where; each new test's and case's failing line on the
unchanged code, then its passing line; each existing test you changed, and why; and every check's
last line. Stage nothing.

## What follows

- **A short engine order**, or a part of the kit's: PRV-001's rejection of a Type listing `run.v2`
  under `requires` without `runtime-provider.v1`, and `run.v2` among PKG-020's and PKG-030's
  provider operations, in `check_items` and `tools/check_schemas.py`, with a case.
- **The kit's and ssh-runtime's** (DEC-049, DEC-080): `core#run.v2` in ssh-runtime's `requires`;
  the archive carried from `urls[0]`, kept by digest on the node, and checked for its length and
  SHA-256 before each unpacking (PRV-019).
- **The packages'** (DEC-051): the two Agent packages for `darwin/arm64`, `linux/amd64` and
  `linux/arm64`, the Linux ones static (musl).
- Found while writing: `place.v1` gets no pairs (order 274), and no source places by a machine
  provider (PRV-015).

## Record

Run on 2026-11-22 through Conduit itself, by the dogfood's own Agents, in Pacific time, stacked
on order 278.

**Worker-1's turn** (13:49 to 14:20) built DEC-080's run side: a package's item that states
`platforms` is placed only on a node whose two labels are one listed pair, and its provider is
sent `run.v2` with the pair's program and the kept archive's digest, size and `file:` URL when the
provider's Type requires it, never `run.v1`. The two portable cases and three shell tests failed
on the unchanged engine and pass, with the 189 core tests and the whole vectors test.

**Two blind reads at once** (14:21 to 14:28), neither with a must: the Codex verifier's two
coverage shoulds (pairs matched as pairs, not as separate memberships; `run.v2` with no URL) and
the Claude reader's two shoulds and two nits (the defensive run refusals as explicit as the
placement one; `engine/CONFORMANCE.md` silent on this order; a file-kind fallback; one form of
the item name in PRV-007's refusals). One fix turn (14:28 to 14:40) applied all six, each new
test failing first under its targeted regression. The CONFORMANCE paragraph lay outside Scope;
the coordinator wrote it from the worker's recorded content. The gate's run on the squash is the
coordinator's, at the merge.
