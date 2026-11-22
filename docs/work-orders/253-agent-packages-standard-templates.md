# Work order 253: the Agent packages export the standard Conduit templates `turn` and `conversation` (PKG-031), released as 0.2.2

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`: list in
`vectors` each case whose files decision 3 changes, with the promises each tests and whether it
passes; you add no case. This order needs no network; its tests bind only loopback ports. Every
crate is in the local cache: run cargo offline, with `CARGO_NET_OFFLINE=true` on each command that
builds a package, since the `package` program and the engine's `agents` tests run cargo themselves;
if a build asks for a missing crate, stop and report, and fetch nothing. Run one cargo command at a
time. Never print a whole large file, case, diff or log: use a count or a slice. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never read or
print a key, a token or any credential. Do not run `conduit` outside a test, the real `codex` or
`claude`, the dogfood harness or `tools/gate.py`, and sign in nowhere. Prefix each `python3` with
`PYTHONDONTWRITEBYTECODE=1`. Every file you write is public, your report included: no absolute local
path, user or machine name; write an archive as `dist/NAME.tar.gz#SHA256`.

## What and why

**The gap.** PKG-031 (`spec/promises/packages.yaml`): a package that exports a capability "also
exports standard Conduit templates for the capability's common uses, named for those uses, such as
`read`, `read-write` and `turn`", so that "Authors can write `from: <address>#read` without copying
an operation list"; ruling 63, point 3, made it the rule. The `git` package and the filesystem
fixture at 0.2.1 export `read` and `read-write`; K3's agents fixture `agents-0.2.1` exports `turn`.
The real Agent packages, `packages/agent-claude-code` and `packages/agent-codex` at 0.2.1, export no
template, so S1's author cannot write root's turn Conduit as `operator: {from: <address>#turn}` and
writes an inline `capability` and `operations`: a second grant form.

**The owner's answer.** On 2026-11-21, to the post-dogfood review's question on root's turn
Conduits: "Templates in Agent pkgs 0.2.2 (Recommended)"
([DEC-040](../decisions/log.md#dec-040-2026-11-21-the-s1-chains-init-templates-and-pins)).
Ruling 71, point 4, left to the owner a template that can move S1's counts.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 2,
every access is a grant that can be inspected and revoked: a Conduit made `from: <address>#turn`
is the same inspectable Conduit an inline one is, its operations fixed by the pinned release
(INS-013); 4, nothing assumes a vendor: both packages export the same two names and lists; 6,
nothing repeats an effect implicitly: `turn` leaves out `new-conversation.v1`, which ends a
conversation (`destructiveHint: true`), and `conversation` is the wider grant, asked for by
name. None at risk: no contract under an existing name changes, and an inline Conduit resolves
as today.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes, by
existing practice: Kubernetes ships the ClusterRoles `view`, `edit` and `admin`, so a grant names a
role instead of listing verbs; and it is PKG-031's own recommendation. Not taken: inline `operator:`
lines on 0.2.1 (a second grant form, against PKG-031 and ruling 63); templates in the author's
project (two more lines in S1's counted file).

**When it runs.** Before DEC-042's login package orders and the second-account sitting after them.
DEC-045 puts those before the S1 chain where they contend; this order delays none, since the login
orders wait for their spec order (CTR-013), and landing first lets the sitting sign in once: an
instance keeps the template it resolved, so moving the store's Agents to a new release replaces and
signs in each one again
([stage 2's manual step 7, gap 13](../../experiments/028-a78-dogfood/results-2026-11-21-stage-2.md#manual-steps)).
Before order 250 too, whose example then writes `from:`. Through Conduit's dogfood: one Codex
worker turn (a 4-hour limit; aim for under 2 hours), a Codex verifier and a Claude reader, who run
only what settles a doubt, and the gate, which runs everything (DEC-030, DEC-033). By ruling 89,
each decision says what must hold; how is yours. Names and words are the coordinator's, open to
the owner's strike.

**Stop rule.** Change nothing under `spec/`. Stop, change nothing further, and report where:

- the engine refuses or mis-resolves a template of decision 1: report the diagnostic;
- a correct result needs a change outside the scope: a Type, an Operation, a wrapper or engine
  source, another case, the install-archive fixtures or a dependency;
- `vector_delta.py` reports a change decision 4 does not declare, or a vector fails: report the
  case, step and path;
- the two `package` runs of one package print different digests;
- a test would start `claude` or `codex`;
- an existing test fails and decision 5's last bullet does not cover its fix;
- a crate is missing from the cache, or a step needs the network.

A choice no decision governs is yours: list it under `own_choices`.

## The decisions

### The release

1. **The templates** (PKG-031; NAM-018, NAM-022). Each package's `contracts.yaml` gains, after
   its last document, these two, byte for byte without the indent:

   ```yaml
   ---
   apiVersion: github.com/shrav-k/conduit/v0
   kind: Conduit
   metadata:
     name: turn
   spec:
     capability: '#conversation.v1'
     operations: ['#turn.v1']
   ---
   apiVersion: github.com/shrav-k/conduit/v0
   kind: Conduit
   metadata:
     name: conversation
   spec:
     capability: '#conversation.v1'
     operations: ['#turn.v1', '#new-conversation.v1']
   ```

   Its `conduit.yaml`'s `exports` gains `- turn` and then `- conversation` after
   `- new-conversation.v1`. Nothing else in either file changes but decision 2's line: no Type,
   Operation, Capability or implementation item, and no wrapper source.
2. **The version** (PKG-001, NAM-024). Each `conduit.yaml`'s `version: 0.2.1` and each
   `Cargo.toml`'s `version = "0.2.1"` become 0.2.2; in each `Cargo.lock` the package's own
   `version` line alone follows (a `--locked` build refuses until it does). NAM-024 holds: 0.2.2
   adds two exports under new names and changes no contract 0.2.1 exported.

### The engine's copies

3. **Copies, archives, pins and cases**, as
   [order 203's decision 7](203-new-conversation.md#the-decisions-it-applies) moved them from
   0.2.0 to 0.2.1, now from 0.2.1 to 0.2.2: both copies under `conformance/fixtures/agents/`,
   byte for byte; their `contracts.tar.gz`, by `make_archives` in write mode; their READMEs, with
   `(work order 253)` for 203's mention; the pins in `live-agents/fixture.json`,
   `engine/examples/s1/conduit.yaml` and `agents.rs`; and the cases
   `registration-agent-v2-packages` and `registration-agent-v2-account-binding`. What 203 kept
   at 0.2.0 stays as it is. Beyond 203's text:
   - In `registration-agent-v2-packages/case.yaml`, each `install-archive` step's `documents`
     array gains, after its last member, `{"kind": "Conduit", "metadata": {"name": "turn"}}` and
     then `{"kind": "Conduit", "metadata": {"name": "conversation"}}`, written as the file writes
     the `turn.v1` member.
   - `make_archives`' 0.2.0 surface leaves the templates out as it leaves `new-conversation.v1`
     out: no `kind: Conduit` document taken from a copy and no `turn` or `conversation` export,
     in the release `agents-0.2.0` (which adds its own `turn`) and in the synthetic
     `codex-no-account-hook`. Every output but the copies' archives keeps its bytes.
   - `engine/examples/s1`'s other files stay: its own Conduits are order 250's.
4. **The declaration**, the new file `docs/work-orders/253-declared.yaml`: the first five shapes
   of `203-declared.yaml`, named for 0.2.2, the version shape with `before: '0.2.1'` and
   `after: '0.2.2'`; then three `add` shapes for `registration-agent-v2-packages`, at paths
   `/steps/*/expect/reply/package/project/documents/` followed by `5/kind` and `6/kind`
   (`after: Conduit`), `5/metadata/name` (`after: turn`) and `6/metadata/name`
   (`after: conversation`). Eight shapes, covering 37 changes: 17 fixture lines, as 203's; 12
   versions and digests; 8 added fields.

### Tests and documents

5. **Tests.** Write the new ones first and run them on the unchanged code; report each failing
   line.
   - `standard_conduit_templates_keep_their_operations`, in each package's `tests/package.rs`:
     `conduit.yaml`'s `exports` end with `turn` then `conversation`, and `contracts.yaml` holds
     each of decision 1's two documents, as text, exactly once, so a later release that changes
     a list fails here (PKG-031: a later release "keeps each such template's operation list").
   - `agent_package_templates_resolve_through_the_engine`, in
     `engine/crates/shell/tests/agents.rs`, for each package: its host archive, as `pack` builds
     it, goes into a new store by `boundary::install_package`, and a short-form project with two
     Agents of the package's `.v2` Type, each with `config: {account: null}`, and the Fleet they
     need is sent as `register-project` through `shell.submit` with `MissingExecutor`, as
     `setup_s1` registers the S1 project. Root's `operator` for one is `{from: ADDRESS#turn}`,
     for the other `{from: ADDRESS#conversation}`. Registration is accepted, and the Conduits
     `NAME--operator` that the store holds (SCT-009, INS-013) have the capability
     `ADDRESS#conversation.v1` and exactly the operations `[ADDRESS#turn.v1]` and
     `[ADDRESS#turn.v1, ADDRESS#new-conversation.v1]`, as the store writes a qualified
     reference. Today registration is refused: 0.2.1 exports no `turn`. The test is hermetic: no
     setup runs and no wrapper or client starts. Not the CLI's `register`: its setup sends an
     `account` string's account hook, which runs the client.
   - `pack` builds each package once per test binary (a `OnceLock`, say): tests run in parallel,
     and each `package` run rewrites the `dist/NAME.tar.gz` another test may be reading.
   - An existing test changes only where it asserts what decisions 1 to 3 change: a version, a
     digest, an `exports` list or a document count. Report each.
6. **The READMEs.** Each package's README ends with a section
   `## The standard Conduit templates`: from 0.2.2, `turn` grants `turn.v1` alone and
   `conversation` both operations; an author writes `operator: {from: ADDRESS#turn}`, with the
   package's address, or `#conversation` where root may also end a conversation; a later release
   keeps each list, and wider use takes a new template (PKG-031); the Types, the Agent's home and
   sign-in are 0.2.1's; an instance keeps the release it resolved, so using 0.2.2 replaces it,
   and a replaced `.v2` Agent signs in again. After "Run these commands", one sentence: built
   offline, the archive's digest is the same on every run with one host and toolchain.

## What to read

- PKG-031, NAM-018, NAM-022, NAM-024, SCT-009 and INS-013 in `spec/promises/`.
- `packages/git/contracts.yaml`'s templates; the `Conduit` documents of the fixtures
  `filesystem-0.2.1` and `agents-0.2.1` in `conformance/fixtures/s1/packages/`.
- [Order 203](203-new-conversation.md)'s decision 7 and Done-when, and `203-declared.yaml`.
- Each package's manifests, contracts, README, `src/bin/package.rs` and `tests/package.rs`;
  `agents.rs` (`pack`, `setup_s1`); `make_archives.rs` (`agent_fixtures`,
  `agent_contracts_v020`); the three cases; the sectioned case `operator-referenced`.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor f7b1af11 HEAD; echo "main $?"
for p in agent-claude-code agent-codex; do d=packages/$p; c=conformance/fixtures/agents/$p
  echo "$p $(grep -c '^version: 0\.2\.1$' $d/conduit.yaml) $(grep -c '^version = "0\.2\.1"$' $d/Cargo.toml) $(grep -cE '^- (turn|conversation)$' $d/conduit.yaml) $(grep -c '^kind: Conduit$' $d/contracts.yaml)"
  for f in conduit.yaml contracts.yaml; do cmp -s $d/$f $c/$f; echo "$p $f $?"; done
  D=$(shasum -a 256 $c/contracts.tar.gz | cut -c1-64); git grep -l "sha256:$D" | wc -l | tr -d ' '
done
grep -c 'version: 0\.2\.1\\n    digest' engine/crates/shell/tests/agents.rs
PYTHONDONTWRITEBYTECODE=1 python3 -c "import json; c=json.load(open('conformance/language/registration/agent-v2-packages/case.yaml')); print(*[len(s['expect']['reply']['package']['project']['documents']) for s in c['steps']])"
git grep -lE 'fn (standard_conduit_templates_keep_their_operations|agent_package_templates_resolve_through_the_engine)\(' | wc -l | tr -d ' '
test -e docs/work-orders/253-declared.yaml; echo "declared $?"
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
for d in engine packages/agent-claude-code packages/agent-codex; do (cd $d && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "$d $?"; done
(cd engine && cargo run --offline --release -p conduit-conformance --example make_archives -- --check 2>&1 | tail -n 1)
git status --short | wc -l | tr -d ' '
```

They print `main 0`; for each package `NAME 1 1 0 0`, `NAME conduit.yaml 0`,
`NAME contracts.yaml 0` and `7`; then `1`, `5 5`, `0`, `declared 1`, three lines ending ` 0`,
`N reproducible archive fixtures verified` (keep N) and `0`. Other counts mean another order changed
these files first: stop and report them.

## Scope

Change, in both packages, `conduit.yaml`, `contracts.yaml`, `Cargo.toml`, `Cargo.lock`,
`README.md` and `tests/package.rs`; both copies' `conduit.yaml`, `contracts.yaml`,
`contracts.tar.gz` and `README.md`; `live-agents/fixture.json`; in the two cases, their install
files, the binding case's `conduit.json` and each `case.yaml`; `engine/examples/s1/conduit.yaml`;
`engine/crates/shell/tests/agents.rs`; `engine/crates/conformance/examples/make_archives.rs`;
and the new `docs/work-orders/253-declared.yaml`. Change nothing else: not `spec/`, a package's
`src/` or other tests, another case or fixture, an engine source, `experiments/`, or a
dependency. Put anything else that needs changing in your report.

## Done when

Each line of the first group prints otherwise on `main` at `f7b1af11` ("today") and passes
after:

```sh
ok='^test result: ok\. 1 passed'
for p in agent-claude-code agent-codex; do d=packages/$p; echo "$p $(grep -c '^version: 0\.2\.2$' $d/conduit.yaml) $(grep -c '^version = "0\.2\.2"$' $d/Cargo.toml) $(grep -cE '^- (turn|conversation)$' $d/conduit.yaml) $(grep -c '^kind: Conduit$' $d/contracts.yaml) $(grep -c '^## The standard Conduit templates$' $d/README.md)"; done  # NAME 1 1 2 2 1 each; today NAME 0 0 0 0 0
for p in agent-claude-code agent-codex; do (cd packages/$p && CARGO_NET_OFFLINE=true cargo test --offline --release --test package standard_conduit_templates_keep_their_operations 2>&1 | grep -c "$ok"); done  # 1 each; today 0
(cd engine && CARGO_NET_OFFLINE=true cargo test --offline --release -p conduit-shell --test agents agent_package_templates_resolve_through_the_engine 2>&1 | grep -c "$ok")  # 1; today 0
PYTHONDONTWRITEBYTECODE=1 python3 -c "import json; c=json.load(open('conformance/language/registration/agent-v2-packages/case.yaml')); print(*[len(s['expect']['reply']['package']['project']['documents']) for s in c['steps']])"  # 7 7; today 5 5
grep -c 'version: 0\.2\.2\\n    digest' engine/crates/shell/tests/agents.rs  # 1; today 0
for p in agent-claude-code agent-codex; do grep -c 'at version 0\.2\.2 (work order 253)' conformance/fixtures/agents/$p/README.md; done  # 1 each; today 0
for p in agent-claude-code agent-codex; do r=; for i in 1 2; do r="$r $(cd packages/$p && CARGO_NET_OFFLINE=true cargo run --offline --locked --release --bin package 2>/dev/null | sed 's|^.*/dist/|dist/|')"; done; echo "$p$r"; tar -xOzf packages/$p/dist/$p.tar.gz conduit.yaml | grep -c '^version: 0\.2\.2$'; done  # NAME, one dist/NAME.tar.gz#SHA256 twice, then 1; today the 1 is 0
out=$(/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/253-declared.yaml); rc=$?; printf '%s\n' "$out" | tail -n 2; echo "exit $rc"  # 8 shapes; 37 changes, 0 undeclared, 0 errors; exit 0; today exit 2
```

Guards, which pass today and must still pass; run the packages' suites, then the whole engine
suite, once each, last:

```sh
for p in agent-claude-code agent-codex; do for f in conduit.yaml contracts.yaml; do cmp -s packages/$p/$f conformance/fixtures/agents/$p/$f; echo "$p $f $?"; tar -xOzf conformance/fixtures/agents/$p/contracts.tar.gz $f | cmp -s - packages/$p/$f; echo "$p archive $f $?"; done; D=$(shasum -a 256 conformance/fixtures/agents/$p/contracts.tar.gz | cut -c1-64); git grep -l "sha256:$D" | wc -l | tr -d ' '; done  # each 0, then 7
(cd engine && cargo run --offline --release -p conduit-conformance --example make_archives -- --check 2>&1 | tail -n 1)  # the start check's line
git diff --name-only HEAD -- spec conformance/fixtures/install-archives conformance/fixtures/s1 conformance/language/packages conformance/language/registration/agent-v2-account-hook-missing engine/examples/s1/templates.yaml engine/examples/s1/instances.yaml | wc -l | tr -d ' '  # 0
for p in agent-claude-code agent-codex; do git diff --numstat HEAD -- packages/$p/Cargo.lock | cut -f1,2; done  # 1 1 each
out=$(/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared-changed); rc=$?; printf '%s\n' "$out" | tail -n 1; echo "exit $rc"  # as above; exit 0
for p in agent-claude-code agent-codex; do (cd packages/$p && cargo fmt --check && CARGO_NET_OFFLINE=true cargo clippy --offline --all-targets -- -D warnings); done
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-shell -p conduit-conformance --all-targets -- -D warnings)
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_docs.py  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/promise_standing.py --check  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_private_paths.py  # Errors: 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(packages/agent-[a-z-]+/(Cargo\.(toml|lock)|[a-z]+\.yaml|README\.md|tests/package\.rs)|conformance/fixtures/agents/agent-[a-z-]+/[a-zA-Z.]+|conformance/language/registration/(live-agents|agent-v2-[a-z-]+)/[a-z-]+\.(json|yaml)|engine/examples/s1/conduit\.yaml|engine/crates/(shell/tests/agents|conformance/examples/make_archives)\.rs|docs/work-orders/253-declared\.yaml)$' | wc -l | tr -d ' '  # 0
for p in agent-claude-code agent-codex; do (cd packages/$p && CARGO_NET_OFFLINE=true cargo test --offline --release); done  # every test passes
(cd engine && CARGO_NET_OFFLINE=true RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release)  # the whole engine suite, every vector among it
```

Your report gives, per decision, what changed and where; each new test's failing line on the
unchanged code, then its passing line; per package, its two `dist/NAME.tar.gz#SHA256` lines and
`rustc -vV`'s `host:` and `release:` lines; each existing test you changed, and why; your own
choices; and every check's last line.

Stage nothing. The verifier's turn follows; then the gate, a read-only Claude reader, and the
merge.

## What follows

Named here and not in this order:

- Order 250's example, which writes root's turn Conduits inline: before 250 dispatches, the
  coordinator rewrites them as `operator: {from: <address>#turn}` on the 0.2.2 pins, and the
  plan-and-apply reference's sentence that leaves the form "for order 250".
- DEC-042's login package orders start from 0.2.2; the dogfood store moves at the
  second-account sitting after them, each Agent re-created and signed in once, and experiment
  028's fleet pins with it. The S1 rerun's lab installs the archives this report records.
- The coordinator's reading, open to the owner's strike: DEC-045's `agent` package takes these
  names and lists over at the address move (DEC-035).

## Record

Not yet run.
