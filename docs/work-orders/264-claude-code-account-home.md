# Work order 264: a Claude Code login is a Pool's `account-home.v1` Resource, one configuration directory per account per machine, released as 0.2.3

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`: list in
`vectors` each case whose files decision 8 changes, with the promises each tests and whether it
passes; you add no case. It needs no network; its tests bind only loopback ports. Run cargo
offline, with `CARGO_NET_OFFLINE=true` on each command that builds a package; if a build asks for
a missing crate, stop and report, and fetch nothing. Run one cargo command at a time. Never print
a whole large file, case, diff or log. Never run `env`, `printenv`, `set` or `export -p`, never
print an environment variable's value, and never read or print a key, a token or any credential,
a test's fake one included. Run no `conduit` outside a test, no real `claude` or `codex`, no
dogfood harness and not `tools/gate.py`, and sign in nowhere: tests use the scripted client and
fake values only. Prefix each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is
public, your report included: no absolute local path, user, machine or account name; write an
archive as `dist/NAME.tar.gz#SHA256`.

## What and why

**The owner's answers.** [DEC-042](../decisions/log.md#dec-042-2026-11-21-model-logins-are-pool-owned-resources):
"Pool-owned login Resource (Recommended)"; for Claude, "Account home per machine (Recommended)".
DEC-047: auto memory stays off. Order 257 wrote DEC-042 into the spec, order 258 into the engine,
and order 259 did Codex's half. Today each `claude-code.v2` Agent signs in on its own, in its
own configuration directory, and again when replaced.

**Claude Code's documentation** (`claude-directory`, `memory` and `authentication` under
code.claude.com/docs/en/, read 2026-11-21). With `CLAUDE_CONFIG_DIR` set, "every `~/.claude` path
on this page lives under that directory instead", `.claude.json` included; "Each directory has
its own settings, session history, and claude.ai login or API key". Auto memory "is on by default
in local sessions", in `projects/<project>/memory/`, where "all worktrees and subdirectories
within the same repo share one auto memory directory". Two documented switches turn it off:
`"autoMemoryEnabled": false` in settings, and `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1`. The `/memory`
toggle writes `autoMemoryEnabled` to user settings, which `--setting-sources ""` leaves out.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 3, the
account's directory is owned once, by the Pool's Resource, never copied; 4, both packages serve
the same shape, and no Type names a plan; 6, an Agent's hooks never log out the shared login,
and a lapse raises one action, never a sign-in. At risk, 2: an Agent's own unconfined tools
inherit `CLAUDE_CONFIG_DIR` and can sign the account out (ruling 70); and the home's files, per
session ones among them, are readable by every Agent of it (PRV-029, DL16). DEC-042 accepted
that cost; the README states it.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)).
Yes, by existing practice: Claude Code's own "Parallel sessions on one machine share a saved
login and coordinate its renewal"; its login lives in its configuration directory, so the shared
directory is the vendor's own sharing. Not taken: a link to one login file, as for Codex (on the
Mac the login is in the keychain, its entry keyed to the directory); a login per Agent (the
owner's flaw).

**When it runs.** On `main` once order 259 merges: both move the same pins and cases. Through
Conduit's dogfood: one Codex worker turn (a 4-hour limit); a Codex verifier and a Claude reader,
blind and at once; one fix turn; the gate (DEC-046). By ruling 89, how is yours. The names,
texts, test names and the reading that auto memory is turned off for `.v2` too (DEC-047) are the
coordinator's, open to the owner's strike.

**Stop rule.** Stop, change nothing further, and report where the engine refuses a Type of
decision 1 or 2; a correct result needs a change outside the scope; `vector_delta.py` reports an
undeclared change or a vector fails; two `package` runs differ; a test would start a real client;
or a crate is missing. A choice no decision governs is yours: list it under `own_choices`.

## The decisions

1. **`account-home.v1`**, a new Type after the last document of `contracts.yaml`, family
   `Resource`: `schema` holds `account`, a required string (CTR-013); `signIn: {account:
   /account, logins: ...}`, its logins `claude-code.v2`'s byte for byte, with Claude Code's client
   ID (sibling Types share it, CTR-013); `requires` `sign-in-hook.v1`, `account-hook.v1`,
   `sign-out-hook.v1` and `retire-hook.v1`, served by a new plain item `login` as the tools
   `sign-in`, `account`, `sign-out` and `retire`; `records: {inputs: false, outputs: false}`; no
   `offers`. Its description: one account's Claude Code configuration directory on one machine,
   its login among it, for its Agents, on any plan, signed in and out only by its own hooks.
2. **`claude-code.v3`**, a new Type after it, family `Agent`: `.v2`'s `schema` without `account`,
   with `login`, required, of schema `github.com/shrav-k/conduit/core#resource-reference.v1`;
   `toolsDirectory: {directory: /directory}`; `loginResource: {place: /login}`; no `signIn`;
   `offers: ['#conversation.v1']`; `requires` `retire-hook.v1`; its three operations served by a
   new item `wrapper-v3`, binding `.v2`'s variables and `AGENT_LOGIN_DIRECTORY: /login`, which
   receives the Resource's directory (PRV-029). The description is `.v2`'s, its last two
   sentences saying instead that the client runs in its login Resource's configuration
   directory, shared by that Resource's Agents, and that the retire hook logs nothing out.
   `conduit.yaml` exports both after `- conversation`. Every existing document and item stays
   byte for byte (NAM-024).
3. **The Resource's hooks**, moved from `.v2`'s, which stay. The `login` run keeps
   `account-home` (0700) in its state directory, with `.v2`'s home checks, prefixed `login home
   (CLAUDE_CONFIG_DIR):`, and runs `.v2`'s four hooks there, with their texts: `claude auth
   login`, `claude auth status`, `claude auth logout`, and retire's logout and emptying.
4. **The `.v3` home.** Every `.v3` launch sets `CLAUDE_CONFIG_DIR` to
   `AGENT_LOGIN_DIRECTORY/account-home`, checked as `.v2` checks its home (absolute, an existing
   directory inside the variable's, valid Unicode), refusals prefixed `login home
   (CLAUDE_CONFIG_DIR):`; the wrapper never creates it. It keeps `.v2`'s arguments,
   `--setting-sources ""`, `--settings`, `--strict-mcp-config` and `--no-session-persistence`
   among them, and writes `settings.json` and `mcp.json` in the run's private directory, never in
   the home, so each Agent's settings, MCP servers and conversation stay its own.
5. **Auto memory off** (DEC-047). Every `.v2` and `.v3` launch writes `"autoMemoryEnabled":
   false` in the settings it passes with `--settings`, and sets `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1`
   in the client's environment: Claude Code's two documented switches, either enough.
6. **The lapse report** (PND-005, IFC-016), in order 258's form; the item stays plain. A `.v3` turn
   that finds the home missing, or whose client fails with class `authentication`, answers with
   exactly 258's `input_required` result: one URL-mode request `sign-in`, its `message` fixed text
   saying that the login Resource's Claude Code login is missing or lapsed, its `url`
   `https://code.claude.com/docs/en/authentication` (the vendor's sign-in page), no other member. No
   other failure answers so, a usage limit included. A lapse after the client started ends the
   conversation as a failed turn does; a missing home ends nothing. The engine records the lapse,
   raises at most one sign-in action and one notice on the Resource, holds no Agent, and runs the
   login's turns one at a time until root lifts it; only the Resource's own hold (PND-005, PND-011)
   holds every Agent (order 258). `.v1` and `.v2` answer as today.
7. **The `.v3` retire hook** stops the tree and refuses later turns as `.v2`'s does, runs no
   `claude auth logout`, removes nothing from the home, and answers `client stopped; its login
   and configuration directory were left to its login Resource`. `.v3` serves no account,
   sign-in or sign-out hook: the engine dispatches none to an Agent of a login Resource
   (order 258).
8. **0.2.3, for this package alone**, as order 259's decision 7 moved `agent-codex`: the version
   lines and the lockfile's own line; the copy under
   `conformance/fixtures/agents/agent-claude-code/`, its archive by `make_archives` in write mode,
   its README `(work order 264)`; the Claude Code pins in `live-agents/fixture.json`,
   `engine/examples/s1/conduit.yaml` and `agents.rs`; and the two `registration-agent-v2-*`
   cases, the Claude Code install step's `documents` gaining `{"kind": "Type", "metadata":
   {"name": "account-home.v1"}}` and then `claude-code.v3`'s, as the file writes the others.
   `make_archives`' 0.2.0 surface leaves out both Types and items; other outputs keep their
   bytes. Declare it all in `docs/work-orders/264-declared.yaml`, as 259's.
9. **Tests**, written first and run on the unchanged code; report each failing line.
   - In `tests/wrapper.rs`: `login_resource_hooks_keep_the_account_home` (decision 3: the four
     hooks' `CLAUDE_CONFIG_DIR` and the home's mode); `v3_runs_in_the_account_home` (decision 4:
     the variable on a first and a resumed turn, each refusal starting no client, `settings.json`
     and `mcp.json` outside the home); `auto_memory_stays_off` (decision 5, for `.v2` and `.v3`,
     on a first turn, a second and one after `new-conversation`, and with the home's own
     `settings.json` planted as `{"autoMemoryEnabled": true}`, as the `/memory` toggle writes it:
     every launch the same); `v3_retire_never_logs_out_its_login_resource` (no logout recorded,
     the home's entries unchanged); `v3_lapsed_login_is_reported_as_the_resources` (decision 6's
     result, member by member, for a missing home and an `authentication` failure; none for a
     usage limit; `.v2` as today).
   - In `tests/package.rs`, `account_home_types_keep_their_shape`: decisions 1 and 2, the logins
     equal to `claude-code.v2`'s.
   - In `agents.rs`, `claude_account_home_registers_through_the_engine`, as 259's Codex test: a
     Pool holding one `account-home.v1` Resource (`account: account-a`) and two
     `claude-code.v3` Agents whose `login` names it; accepted, each login place holding the
     Resource's resolved reference. Today 0.2.2 exports neither Type.
   - Change an existing test only where decisions 1 to 8 require; report each.
10. **The README.**
    - Before `## The standard Conduit templates`: `## The account home Resource` (decisions 1
      and 3) and `## The .v3 Agent` (decisions 2 and 4 to 7). The latter states what the home
      shares among its Agents and what each keeps: its conversation (`--no-session-persistence`
      keeps no transcript), settings and MCP servers; shared, auto memory, off; `.claude.json`
      (account details, project trust) and `stats-cache.json` (the `/usage` totals; DEC-047
      measures per turn); per-session files such as `file-history/`, `session-env/` and
      `sessions/`, readable by every Agent of the home (PRV-029, DL16); and any Agent's tools can
      sign the account out (ruling 70).
    - The templates section (order 253's Record): the `#conversation` form written as
      `operator: {from: ...#conversation}`, in `turn`'s words, and a link to the README's
      section on `new-conversation.v1`.
    - Last, `## Moving to 0.2.3`: install the archive, move the pin, replace each Agent, a
      replaced `.v2` Agent signing in again; for `.v3`, retire each `.v2` Agent first (it logs
      out its own login), sign in each account's account home once per machine, then register
      the `.v3` Agents.

## What to read

CTR-013, PRV-029 and PND-005 as order 257 writes them; order 258 (its form of a plain turn's
lapse) and IFC-016; order 259 and its declaration; this package; `agents.rs`;
`make_archives.rs`.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
j spec/promises/contracts.yaml 'states signIn or loginResource.'
grep -cE '^(version: 0\.2\.3|- codex\.v3)$' packages/agent-codex/conduit.yaml
d=packages/agent-claude-code; c=conformance/fixtures/agents/agent-claude-code
echo "$(grep -c '^version: 0\.2\.2$' $d/conduit.yaml) $(grep -c '^version = "0\.2\.2"$' $d/Cargo.toml) $(grep -cE '^- (account-home\.v1|claude-code\.v3)$' $d/conduit.yaml) $(grep -c '^kind: Conduit$' $d/contracts.yaml)"
for f in conduit.yaml contracts.yaml; do cmp -s $d/$f $c/$f; echo "$f $?"; done
D=$(shasum -a 256 $c/contracts.tar.gz | cut -c1-64); git grep -l "sha256:$D" | wc -l | tr -d ' '
PYTHONDONTWRITEBYTECODE=1 python3 -c "import json; c=json.load(open('conformance/language/registration/agent-v2-packages/case.yaml')); print(*[len(s['expect']['reply']['package']['project']['documents']) for s in c['steps']])"
git grep -l -e autoMemoryEnabled -e CLAUDE_CODE_DISABLE_AUTO_MEMORY -- $d | wc -l | tr -d ' '
test -e docs/work-orders/264-declared.yaml; echo "declared $?"
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
for p in engine $d; do (cd $p && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "$p $?"; done
(cd engine && cargo run --offline --release -p conduit-conformance --example make_archives -- --check 2>&1 | tail -n 1)
git status --short | wc -l | tr -d ' '
```

They print `1`, `2`, `1 1 0 2`, `conduit.yaml 0`, `contracts.yaml 0`, `7`, `7 9`, `0`,
`declared 1`, two lines ending ` 0`, `N reproducible archive fixtures verified` (keep N) and `0`.

## Scope

Change, in `packages/agent-claude-code`, `conduit.yaml`, `contracts.yaml`, `Cargo.toml`,
`Cargo.lock`, `README.md`, `src/` and `tests/`; the copy's `conduit.yaml`, `contracts.yaml`,
`contracts.tar.gz` and `README.md`; `live-agents/fixture.json`; the two cases' Claude Code
install files, the binding case's `conduit.json` and each `case.yaml`;
`engine/examples/s1/conduit.yaml`; `agents.rs`; `make_archives.rs`; and the new
`docs/work-orders/264-declared.yaml`. Change nothing else: not `spec/`, `packages/agent-codex` or
its copy, another case or fixture, an engine source, `experiments/` or a dependency; report what
else needs it.

## Done when

Each line of the first group prints otherwise on `main` after order 259 ("today") and passes
after:

```sh
ok='^test result: ok\. 1 passed'
d=packages/agent-claude-code; c=conformance/fixtures/agents/agent-claude-code
echo "$(grep -c '^version: 0\.2\.3$' $d/conduit.yaml) $(grep -c '^version = "0\.2\.3"$' $d/Cargo.toml) $(grep -cE '^- (account-home\.v1|claude-code\.v3)$' $d/conduit.yaml) $(grep -cE '^  name: (account-home\.v1|claude-code\.v3)$' $d/contracts.yaml) $(grep -cE '^## (The account home Resource|The \.v3 Agent|Moving to 0\.2\.3)$' $d/README.md)"  # 1 1 2 2 3; today 0 0 0 0 0
echo "$(grep -c 'operator: {from: example.org/conduit/agent-claude-code#conversation}' $d/README.md) $(sed -n '/^## The standard Conduit templates$/,/^## M/p' $d/README.md | grep -c '(#a-new-conversation-both-versions)')"  # 1 1; today 0 0
for t in login_resource_hooks_keep_the_account_home v3_runs_in_the_account_home auto_memory_stays_off v3_retire_never_logs_out_its_login_resource v3_lapsed_login_is_reported_as_the_resources; do (cd $d && CARGO_NET_OFFLINE=true cargo test --offline --release --test wrapper $t 2>&1 | grep -c "$ok"); done  # 1 each; today 0
(cd $d && CARGO_NET_OFFLINE=true cargo test --offline --release --test package account_home_types_keep_their_shape 2>&1 | grep -c "$ok")  # 1; today 0
(cd engine && CARGO_NET_OFFLINE=true cargo test --offline --release -p conduit-shell --test agents claude_account_home_registers_through_the_engine 2>&1 | grep -c "$ok")  # 1; today 0
PYTHONDONTWRITEBYTECODE=1 python3 -c "import json; c=json.load(open('conformance/language/registration/agent-v2-packages/case.yaml')); print(*[len(s['expect']['reply']['package']['project']['documents']) for s in c['steps']])"  # 9 9; today 7 9
echo "$(grep -c 'at version 0\.2\.3 (work order 264)' $c/README.md) $(grep -c 'version: 0\.2\.3' engine/examples/s1/conduit.yaml)"  # 1 2; today 0 1
r=; for i in 1 2; do r="$r $(cd $d && CARGO_NET_OFFLINE=true cargo run --offline --locked --release --bin package 2>/dev/null | sed 's|^.*/dist/|dist/|')"; done; echo "$r"; tar -xOzf $d/dist/agent-claude-code.tar.gz conduit.yaml | grep -c '^version: 0\.2\.3$'  # one dist/agent-claude-code.tar.gz#SHA256 twice, then 1; today the 1 is 0
out=$(/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/264-declared.yaml); rc=$?; printf '%s\n' "$out" | tail -n 2; echo "exit $rc"  # 0 undeclared, 0 errors; exit 0; today exit 2
```

Guards, which pass today and must still pass; run the package's suite, then the whole engine
suite, once each, last:

```sh
d=packages/agent-claude-code; c=conformance/fixtures/agents/agent-claude-code
for f in conduit.yaml contracts.yaml; do cmp -s $d/$f $c/$f; echo "$f $?"; tar -xOzf $c/contracts.tar.gz $f | cmp -s - $d/$f; echo "archive $f $?"; done; D=$(shasum -a 256 $c/contracts.tar.gz | cut -c1-64); git grep -l "sha256:$D" | wc -l | tr -d ' '  # each 0, then 7
(cd engine && cargo run --offline --release -p conduit-conformance --example make_archives -- --check 2>&1 | tail -n 1)  # the start check's line
git diff --numstat HEAD -- $d/Cargo.lock | cut -f1,2  # 1 1
out=$(/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared-changed); rc=$?; printf '%s\n' "$out" | tail -n 1; echo "exit $rc"  # as above; exit 0
(cd $d && cargo fmt --check && CARGO_NET_OFFLINE=true cargo clippy --offline --all-targets -- -D warnings)
(cd engine && cargo fmt --all -- --check && cargo clippy --offline -p conduit-shell -p conduit-conformance --all-targets -- -D warnings)
for t in check_decisions.py\ HEAD\ WORKTREE check_docs.py promise_standing.py\ --check check_private_paths.py; do PYTHONDONTWRITEBYTECODE=1 python3 tools/$t | tail -1; done  # Errors: 0 each
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(packages/agent-claude-code/(Cargo\.(toml|lock)|[a-z]+\.yaml|README\.md|src/.+|tests/.+)|conformance/fixtures/agents/agent-claude-code/[a-zA-Z.]+|conformance/language/registration/(live-agents|agent-v2-[a-z-]+)/[a-z-]+\.(json|yaml)|engine/examples/s1/conduit\.yaml|engine/crates/(shell/tests/agents|conformance/examples/make_archives)\.rs|docs/work-orders/264-declared\.yaml)$' | wc -l | tr -d ' '  # 0
(cd $d && CARGO_NET_OFFLINE=true cargo test --offline --release)  # every test passes
(cd engine && CARGO_NET_OFFLINE=true RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release)  # the whole engine suite, every vector among it
```

Report, per decision, what changed; each new test's failing, then passing line; both
`dist/agent-claude-code.tar.gz#SHA256` lines with `rustc -vV`'s `host:` and `release:`; each
existing test changed, and why; and every check's last line. Stage nothing.

## What follows

- The second-account sitting (DEC-042), README's steps; its first real turn checks that the
  account home holds no `projects/*/memory/` afterwards, the real client's side of decision 5.
- Order 260 (DEC-047's usage and limits) starts from both packages' 0.2.3.

## Record

Not yet run.
