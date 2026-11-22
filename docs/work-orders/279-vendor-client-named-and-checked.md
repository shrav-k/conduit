# Work order 279: each Agent package names its vendor client and the lowest version tested, and its setup hook gives the client found and fails below that version, released as 0.2.5 of both (DEC-049)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`: list in
`vectors` each case decision 6 changes, its promises and whether it passes; add no case. It needs
no network; its tests bind only loopback ports. Run cargo offline, with `CARGO_NET_OFFLINE=true` on
each command that builds a package, and fetch nothing. Run one cargo command at a time. Never print
a whole large file, case, diff or log. Never run `env`, `printenv`, `set` or `export -p`, never
print an environment variable's value, and never read or print a key, a token or any credential.
Run no `conduit` outside a test, no real `codex` or `claude`, no dogfood harness and not
`tools/gate.py`; download and install nothing, and sign in nowhere. Tests use each package's
scripted client (`tests/fixtures/client.rs`) alone. Prefix each `python3` with
`PYTHONDONTWRITEBYTECODE=1`. Every file you write is public, your report included: no absolute
local path, user, machine or account name; write an archive as `dist/NAME.tar.gz#SHA256`.

## What and why

**The owner's answer.** [DEC-049](../decisions/log.md#dec-049-2026-11-21-per-platform-programs-in-one-archive)
lists this order among its five. Each Agent package carries its wrapper, but not the vendor
client (`codex`, `claude`). The wrapper finds the client on the PATH of the machine where it runs,
and the vendor installs and updates it. Under order 274's spec, another runtime provider may run
a package's program on a node (PRV-019, `run.v2`), and the client is then the node's. Today
nothing names the versions a release was tested with. Each turn's usage names the client's version
(DEC-047); nothing gives its path or refuses an older one.

**The coordinator's decision**, open to the owner's strike (DEC-029), under rulings 40 (DEC-003)
and 52 (DEC-010): each package's README names the client, the lowest version this release was
tested with, and that the digest does not cover the client (a Type names no client version, ruling
82, point 5, and NAM-024 keeps its words); the wrapper, which the archive carries, holds the bound;
its setup hook gives the absolute path and version of the client it found, and fails below that
version, naming both; it changes no update setting, and installing stays the vendor's route.

**Read against `main` at `8dfec047`.**

- `codex.v3` and `codex.v4` require `setup-hook.v1`; their items' `setup` tool links the home to
  the login file. No `claude-code` Type requires the hook, and NAM-024 adds none to an existing
  Type, so `claude-code.v5` is `.v4` with the hook.
- An account check (TPL-012) cannot carry the check: one that fails counts as succeeded with no
  account, and raises the sign-in action.
- The hook is the setup's last part (TPL-017), with no outputs, so its result is text (IFC-025).
  A failed hook leaves the setup failed, or unknown where it started the run (OPS-022). A hook
  that succeeded is not dispatched again (OPS-028).
- The tested versions: Codex 0.167.1, which A5 stage 1 ran, the README's source reads cover and
  the scripted client gives; Claude Code 2.1.318, which experiment 027 ran and the README names
  as its baseline. Order 260's `codex --version` probe (`usage.rs`) is the form.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 4,
nothing assumes a vendor's agent: each package states its own client, in one shared form; 6,
nothing starts model work: the check runs one `--version` per hook and installs and updates
nothing. None at risk.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes,
by existing practice. Cargo refuses a package whose `rust-version` is above the toolchain's, naming
both versions. A Kubernetes Node reports its runtime's version in `nodeInfo.containerRuntimeVersion`.
Not taken (the coordinator's design notes): the provider running the vendor's installer (the
vendor's job; it fails offline); turning off Claude Code's updater (a vendor default overridden
on the owner's Mac); Claude Code's managed `requiredMinimumVersion` (machine-wide, binding the
owner's own use, no Codex counterpart); the client pinned in the archive (v1, an `oci` item); the
hook on `wrapper-v4` for a template to select (a setup would check only where its template asks).

**When it runs.** On `main` from `8dfec047` (#550), with order 274 stacked. Through Conduit: one
Codex worker turn (4-hour limit; aim for under 2 hours), a blind Codex verifier and Claude reader
at once, one fix turn, then the gate (DEC-046, DEC-050, DEC-033). By ruling 89, each decision says
what must hold; how is yours. The words, versions, SemVer rule and `claude-code.v5` are the
coordinator's, open to strike. This order builds the host programs only. The packages
order builds the Linux programs in Docker
([DEC-051](../decisions/log.md#dec-051-2026-11-22-the-linux-programs-built-in-docker)), so add no
dependency and nothing that builds on macOS alone.

**Stop rule.** Stop, change nothing further, and report where a test would run a real client or
let the wrapper search a PATH the test did not make; the engine refuses `claude-code.v5`; a
correct result needs a change outside the scope; `vector_delta.py` reports an undeclared change or
a vector fails; two `package` runs differ; or a crate is missing. A choice no decision governs is
yours: list it under `own_choices`.

## The decisions

1. **The client line.** NAME is `codex` or `claude`, and MIN is the package's lowest tested
   version.
   - P is the absolute path of the file the wrapper runs as the client. With `--client-program`,
     P is that path, made absolute. Otherwise P is the first entry of the client's PATH that holds
     an executable regular file named NAME, as the launch finds it. Links are not resolved.
   - V comes from running P with `--version` alone, in the client's environment. The run must
     exit with status 0 within 5 seconds and print at most 256 bytes. For Codex, V is the word
     after `codex-cli `, order 260's rule. For Claude Code, V is the first word; more text may
     follow. V is read only where it is a SemVer 2.0.0 version.
   - Versions compare by SemVer precedence: `0.167.1-alpha.1` is older than `0.167.1`, and
     `0.168.0` is not.

   The four texts, each the whole of one `text` block:

   - `client: NAME V at P; lowest tested MIN`, where V is MIN or newer;
   - `client: NAME V at P is older than MIN, the lowest version this release was tested with; update it by the vendor's own route`;
   - `client: NAME at P gave no version this release can read from --version; the lowest version it was tested with is MIN`;
   - `client: NAME not found on PATH; install MIN or later by the vendor's own route`.

   Each of the last three fails the hook: `isError` is true, that text is the whole result, and
   the hook does nothing else.
2. **Codex** (`agent-codex`). MIN is `0.167.1`. The `setup` tool of `wrapper-v3` and
   `wrapper-v4` keeps its configuration and retirement refusals first, then checks the client
   before it links. On success `content` gives today's text, then the client line as a second
   text block; on failure it links nothing. `contracts.yaml` keeps its bytes.
3. **Claude Code** (`agent-claude-code`). MIN is `2.1.318`.
   - `claude-code.v5`, a new Type after `claude-code.v4`, exported after it: `.v4`'s document,
     `requires` listing `setup-hook.v1` before `retire-hook.v1`, each operation and hook
     defaulting to a new item `wrapper-v5`: `wrapper-v4`'s command and bindings, also
     implementing `setup-hook.v1` as `setup`. Its description is `.v4`'s and: "The setup hook
     gives the path and version of the client it finds, and fails where the client is missing,
     gives no version, or is older than the lowest version the package was tested with." Every
     other document and item keeps its bytes (NAM-024).
   - `setup` runs decision 1 alone, refusing a configuration error and a retired Agent as Codex's
     does. Whether runs of `wrapper-v3` and `wrapper-v4` list it is yours.
4. **No update setting.** Neither wrapper adds, names or changes any client update or version
   setting (Claude Code's `DISABLE_AUTOUPDATER`, `DISABLE_UPDATES` and `autoUpdates`, a minimum
   version, Codex's update check); the client's environment stays as today. Neither installs or
   fetches a client.
5. **The README**, each, before `## The standard Conduit templates`:
   - `## The vendor client`, with "The lowest client version this release was tested with is
     Codex 0.167.1." (or "is Claude Code 2.1.318.") and "The package's digest covers the wrapper
     and this bound, not the client." word for word. It says the client is found on the PATH of
     the machine where the wrapper runs (PRV-019); the vendor's route installs and updates it
     (https://github.com/openai/codex; https://code.claude.com/docs/en/setup, whose native
     install updates itself, left so); decision 1's texts; that Codex `.v3` and `.v4` and Claude
     Code `.v5` check, and other Types and the login Resources do not (TPL-012); that a failed
     setup's reply names the hook part, not its text, until a later order; and that a failed hook
     runs again at the next setup, but one that succeeded does not, so a later client is checked
     when its Agent is replaced (OPS-021, OPS-028). The first paragraph names the bound in place
     of its launch baseline.
   - For Claude Code, `## The .v5 Agent` gives decision 3.
   - Last, `## Moving to 0.2.5`: install the archive, move the pin, and replace each Agent that
     should check its client. A Claude `.v4` Agent replaced by a `.v5` one with the same `login`
     signs in nowhere.
6. **0.2.5 of both**, as orders 260's and 269's decision 7 moved each package: the version lines
   and the lockfile's own line; each copy under `conformance/fixtures/agents/`, its archive by
   `make_archives` in write mode, its README's `at version 0.2.5 (work order 279)`; both pins in
   `live-agents/fixture.json`, `engine/examples/s1/conduit.yaml` and `agents.rs`; and the two
   `registration-agent-v2-*` cases, the packages case's Claude Code install step's `documents`
   gaining `{"kind": "Type", "metadata": {"name": "claude-code.v5"}}` after `.v4`'s.
   `make_archives`' 0.2.0 surface leaves out `claude-code.v5` (`shared_login_type`). Declare it
   all in `docs/work-orders/279-declared.yaml`, as 269's.
7. **Tests.** Write them first, run them on the unchanged code, and report each failing line.
   - In each `tests/wrapper.rs`, `setup_reports_and_checks_the_client` (Codex, `.v3` and `.v4`)
     and `v5_setup_reports_and_checks_the_client` (Claude Code), the scripted client answering
     `--version` in decision 1's form: MIN and a newer version succeed with the first
     text, P the scripted client's absolute path; an older version, a pre-release of MIN, a word
     that is not SemVer, a non-zero exit and silence past 5 seconds fail with decision 1's texts;
     Codex links only on success. Without `--client-program`, with the wrapper's whole PATH one
     temporary directory, a scripted client named NAME there is P, and none gives the missing
     text.
   - In Claude Code's `tests/package.rs`, `claude_code_v5_keeps_its_shape`: decision 3.
   - In `agents.rs`, `claude_code_v5_registers_through_the_engine`, as order 269's `.v4` test: a
     Pool's `account-home.v1` Resource and a `claude-code.v5` Agent register, its resolved
     template selecting `wrapper-v5` for `setup-hook.v1`.
   - Change an existing test only where decisions 1 to 6 require it.

## What to read

TPL-012, TPL-017, OPS-022, OPS-028, IFC-025, NAM-024, PRV-019; orders 260 and 269; both
packages, Codex's `usage.rs` and `setup` above all; `make_archives.rs`; `agents.rs`.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor 8dfec047 HEAD; echo "main $?"
grep -c '^  - run.v2$' spec/core/conduit.yaml
x=packages/agent-codex; y=packages/agent-claude-code
for d in $x $y; do echo "$(grep -c '^version: 0\.2\.4$' $d/conduit.yaml) $(grep -c '^version = "0\.2\.4"$' $d/Cargo.toml)"; done
echo "$(grep -cE '^- claude-code\.v[45]$' $y/conduit.yaml) $(grep -c "auth.json linked to the login Resource's file" $x/src/lib.rs) $(grep -c 'codex-cli 0\.167\.1' $x/tests/fixtures/client.rs)"
for p in agent-codex agent-claude-code; do d=packages/$p; c=conformance/fixtures/agents/$p; for f in conduit.yaml contracts.yaml; do cmp -s $d/$f $c/$f; printf '%s ' $?; done; D=$(shasum -a 256 $c/contracts.tar.gz | cut -c1-64); git grep -l "sha256:$D" | wc -l | tr -d ' '; done
PYTHONDONTWRITEBYTECODE=1 python3 -c "import json; c=json.load(open('conformance/language/registration/agent-v2-packages/case.yaml')); print(*[len(s['expect']['reply']['package']['project']['documents']) for s in c['steps']])"
git grep -n -i -E 'autoupdat|DISABLE_UPDATES|check_for_update|minimumVersion|lowest tested' -- $x/src $y/src | wc -l | tr -d ' '
git grep -lE 'fn (setup_reports_and_checks_the_client|v5_setup_reports_and_checks_the_client|claude_code_v5_[a-z_]+)\(' | wc -l | tr -d ' '
test -e docs/work-orders/279-declared.yaml; echo "declared $?"
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
for p in engine $x $y; do (cd $p && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "$p $?"; done
(cd engine && cargo run --offline --release -p conduit-conformance --example make_archives -- --check 2>&1 | tail -n 1)
git status --short | wc -l | tr -d ' '
```

They print `main 0`, `1` (order 274), `1 1` twice, `1 1 1`, `0 0 7` twice, `10 10`, `0`, `0`,
`declared 1`, three lines ending ` 0`, `N reproducible archive fixtures verified` (keep N) and `0`.

## Scope

Change, in both packages, `conduit.yaml`, `Cargo.toml`, `Cargo.lock`, `README.md`, `src/` and
`tests/`, and Claude Code's `contracts.yaml`; each copy's `conduit.yaml`, `contracts.yaml`,
`contracts.tar.gz` and `README.md`; `live-agents/fixture.json`; the two cases' install files, the
binding case's `conduit.json` and each `case.yaml`; `engine/examples/s1/conduit.yaml`,
`agents.rs` and `make_archives.rs`; and the new `docs/work-orders/279-declared.yaml`. Change
nothing else: not `spec/`, Codex's `contracts.yaml`, another case or fixture, an engine source,
`experiments/` or a dependency. Report what else needs changing.

## Done when

Each line of the first group prints otherwise on `main` with order 274 stacked ("today"), and
passes after:

```sh
ok='^test result: ok\. 1 passed'; x=packages/agent-codex; y=packages/agent-claude-code
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
for d in $x $y; do echo "$(grep -c '^version: 0\.2\.5$' $d/conduit.yaml) $(grep -c '^version = "0\.2\.5"$' $d/Cargo.toml) $(grep -cE '^## (The vendor client|Moving to 0\.2\.5)$' $d/README.md) $(j $d/README.md "The package's digest covers the wrapper and this bound, not the client.")"; done  # 1 1 2 1 twice; today 0 0 0 0
echo "$(j $x/README.md 'tested with is Codex 0.167.1.') $(j $y/README.md 'tested with is Claude Code 2.1.318.') $(grep -c '^- claude-code\.v5$' $y/conduit.yaml) $(grep -c '^  name: claude-code\.v5$' $y/contracts.yaml) $(grep -c '^## The \.v5 Agent$' $y/README.md)"  # 1 1 1 1 1; today 0 0 0 0 0
(cd $x && CARGO_NET_OFFLINE=true cargo test --offline --release --test wrapper setup_reports_and_checks_the_client 2>&1 | grep -c "$ok")  # 1; today 0
(cd $y && CARGO_NET_OFFLINE=true cargo test --offline --release --test wrapper v5_setup_reports_and_checks_the_client 2>&1 | grep -c "$ok")  # 1; today 0
(cd $y && CARGO_NET_OFFLINE=true cargo test --offline --release --test package claude_code_v5_keeps_its_shape 2>&1 | grep -c "$ok")  # 1; today 0
(cd engine && CARGO_NET_OFFLINE=true cargo test --offline --release -p conduit-shell --test agents claude_code_v5_registers_through_the_engine 2>&1 | grep -c "$ok")  # 1; today 0
PYTHONDONTWRITEBYTECODE=1 python3 -c "import json; c=json.load(open('conformance/language/registration/agent-v2-packages/case.yaml')); print(*[len(s['expect']['reply']['package']['project']['documents']) for s in c['steps']])"  # 11 10; today 10 10
echo "$(grep -c 'at version 0\.2\.5 (work order 279)' conformance/fixtures/agents/agent-*/README.md | cut -d: -f2 | tr '\n' ' ')$(grep -c 'version: 0\.2\.5' engine/examples/s1/conduit.yaml)"  # 1 1 2; today 0 0 0
for p in agent-codex agent-claude-code; do d=packages/$p; r=; for i in 1 2; do r="$r $(cd $d && CARGO_NET_OFFLINE=true cargo run --offline --locked --release --bin package 2>/dev/null | sed 's|^.*/dist/|dist/|')"; done; echo "$r $(tar -xOzf $d/dist/$p.tar.gz conduit.yaml | grep -c '^version: 0\.2\.5$')"; done  # per package one dist/NAME.tar.gz#SHA256 twice, then 1; today the 1 is 0
git diff --numstat HEAD -- $x/Cargo.lock $y/Cargo.lock | cut -f1,2 | tr '\n' ' '; git diff --quiet HEAD -- $x/contracts.yaml; echo "codex contracts $?"  # 1 1 1 1 codex contracts 0; today codex contracts 0
git diff --numstat HEAD -- $y/conduit.yaml $y/contracts.yaml | cut -f2 | tr '\n' ' '; echo  # 1 0: the version line; today empty
out=$(/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/279-declared.yaml); rc=$?; printf '%s\n' "$out" | tail -n 2; echo "exit $rc"  # 0 undeclared, 0 errors; exit 0; today exit 2
```

Guards, which pass today and after; run the two package suites and the engine suite once each,
last:

```sh
x=packages/agent-codex; y=packages/agent-claude-code
for p in agent-codex agent-claude-code; do d=packages/$p; c=conformance/fixtures/agents/$p; for f in conduit.yaml contracts.yaml; do cmp -s $d/$f $c/$f; printf '%s ' $?; tar -xOzf $c/contracts.tar.gz $f | cmp -s - $d/$f; printf '%s ' $?; done; D=$(shasum -a 256 $c/contracts.tar.gz | cut -c1-64); git grep -l "sha256:$D" | wc -l | tr -d ' '; done  # 0 0 0 0 7 twice
(cd engine && cargo run --offline --release -p conduit-conformance --example make_archives -- --check 2>&1 | tail -n 1)  # the start check's line
git grep -n -i -E 'autoupdat|DISABLE_UPDATES|check_for_update|minimumVersion' -- $x/src $y/src | wc -l | tr -d ' '  # 0
out=$(/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared-changed); rc=$?; printf '%s\n' "$out" | tail -n 1; echo "exit $rc"  # as above; exit 0
for d in $x $y; do (cd $d && cargo fmt --check && CARGO_NET_OFFLINE=true cargo clippy --offline --all-targets -- -D warnings); done
(cd engine && cargo fmt --all -- --check && cargo clippy --offline -p conduit-shell -p conduit-conformance --all-targets -- -D warnings)
for t in check_decisions.py\ HEAD\ WORKTREE check_docs.py promise_standing.py\ --check check_private_paths.py; do PYTHONDONTWRITEBYTECODE=1 python3 tools/$t | tail -1; done  # Errors: 0 each
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(packages/agent-(codex|claude-code)/(Cargo\.(toml|lock)|[a-z]+\.yaml|README\.md|src/.+|tests/.+)|conformance/fixtures/agents/agent-(codex|claude-code)/[a-zA-Z.]+|conformance/language/registration/(live-agents|agent-v2-[a-z-]+)/[a-z-]+\.(json|yaml)|engine/examples/s1/conduit\.yaml|engine/crates/(shell/tests/agents|conformance/examples/make_archives)\.rs|docs/work-orders/279-declared\.yaml)$' | wc -l | tr -d ' '  # 0
for d in $x $y; do (cd $d && CARGO_NET_OFFLINE=true cargo test --offline --release); done  # every test passes
(cd engine && CARGO_NET_OFFLINE=true RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release)  # the whole engine suite, every vector among it
```

Report per decision what changed; each new test's failing, then passing line; the four
`dist/NAME.tar.gz#SHA256` lines with `rustc -vV`'s `host:` and `release:`; each existing test
changed, and why; every check's last line. Stage nothing.

## What follows

- DEC-049's packages order, whose Linux programs (DEC-051) carry this check to each node; and
  the role templates (DEC-047), the readers moving to `claude-code.v5`.
- How the engine shows the hook's text to the setup's caller (ruling 52); these Types record
  no outputs.

## Record

Run on 2026-11-22 through Conduit itself, by the dogfood's own Agents, in Pacific time, on
`main` after order 274 merged.

**Worker-2's turn** (10:11 to 10:28) released agent-codex and agent-claude-code 0.2.5: each
contract names its vendor client, the lowest version tested and that the package's digest does
not cover the client; each setup probes the selected client once with `--version`, bounded in
time and output, gives its absolute path and version in the result, and fails a client below the
tested minimum naming both versions; neither wrapper changes an update setting or installs a
client. `claude-code.v5` adds the setup hook. The regressions failed first; both packages'
suites and the three repinned cases pass, and both archives reproduce byte for byte.

**Two blind reads at once** (10:28 to 10:33): the Claude reader passed it with no findings;
`turn-verifier-2.turn.v1` found three musts. One fix turn (10:33 to 10:40) applied them, each
reproduced first: the probe runs with the client home the Agent uses on a turn; an invalid login
binding is refused before any probe; a probe that times out ends its process group and is
reaped. Both suites and the cases pass.

The gate's run on the squash is the coordinator's, at the merge.
