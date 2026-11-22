# Work order 259: a Codex login is a Pool's `codex-login.v1` Resource, linked into each Agent's own home, for every plan, released as 0.2.3

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`: list in
`vectors` each case decision 7 changes, its promises and whether it passes; add no case. It needs no
network; its tests bind only loopback ports. Run cargo offline, with `CARGO_NET_OFFLINE=true` on
each command that builds a package, and fetch nothing. Run one cargo command at a time. Never print
a whole large file, case, diff or log. Never run `env`, `printenv`, `set` or `export -p`, never
print an environment variable's value, and never read or print a key, a token or any credential, a
test's fake one included. Run no `conduit` outside a test, no real `codex` or `claude`, no dogfood
harness and not `tools/gate.py`, and sign in nowhere: tests use the scripted client and fake tokens
only. Prefix each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is public, your
report included: no absolute local path, user, machine or account name; write an archive as
`dist/NAME.tar.gz#SHA256`.

## What and why

**The owner's answers.** [DEC-042](../decisions/log.md#dec-042-2026-11-21-model-logins-are-pool-owned-resources):
"Pool-owned login Resource (Recommended)"; for Codex, "we should support every subscription
type". DEC-046: "One login per account, shared", turns in parallel with detection and a
fallback; "OpenAI's word + live watch (Recommended)". Orders 257 and 258 wrote them into the spec
and the engine.

**The design** (the coordinator's study, at Codex `rust-v0.167.1`). The ChatGPT
login has no branch on the plan. One `auth.json` per account per machine sits in the Resource's
home; in each Agent's own home only `auth.json` is a symlink to it. Codex follows the link,
renaming a temp file over the target under the target's lock, and reloads before a refresh. The
exchange is not guarded: an OpenAI maintainer says a reused refresh token is accepted "on the
order of an hour" (Codex issue 10332); OpenAI's CI page says "Do not share the same file across
concurrent jobs". Agent Identity registration
(`use_agent_identity`, off by default) saves without the refresh's comparison.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 3, the
login is owned once, by the Pool's Resource, and linked, never copied; 4, no Type names a plan;
6, an Agent's hooks never log out or sign in the shared login, and a lapse raises one action,
never a sign-in. At risk, 2: an Agent's own unconfined tools inherit `CODEX_HOME` and can run
`codex logout` through the link (ruling 70).

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes, by
existing practice: Codex, "The CLI and extension share the same cached login details". Not taken:
one Codex home per account (an unlocked shared `config.toml`, history, transcripts); a copy per home
(the first refresh leaves the others stale); the keyring (one entry per home path); one turn at a
time from the start (DEC-046).

**When it runs.** On `main` once orders 257 and 258 merge. Through Conduit's dogfood: one Codex
worker turn (a 4-hour limit); a Codex verifier and a Claude reader, blind and at once; one fix turn;
the gate (DEC-046). Order 264, Claude Code's half, follows: both move the same pins. By ruling 89,
how is yours. The names, texts and the readings that `.v3` serves no sign-out hook and that a
dangling link is a missing login are the coordinator's, open to the owner's strike.

**Stop rule.** Stop, change nothing further, and report where the engine refuses a Type of
decision 1 or 2; a correct result needs a change outside the scope; `vector_delta.py` reports an
undeclared change or a vector fails; two `package` runs differ; a test would start a real
client; or a crate is missing. A choice no decision governs is yours: list it under
`own_choices`.

## The decisions

1. **`codex-login.v1`**, a new Type after the last document of `contracts.yaml`, family `Resource`:
   `schema` holds `account`, a required string, the expected account (CTR-013); `signIn: {account:
   /account, logins: ...}`, its logins `codex.v2`'s byte for byte, with Codex's client ID (sibling
   Types share it, CTR-013); `requires` `sign-in-hook.v1`, `account-hook.v1`, `sign-out-hook.v1` and
   `retire-hook.v1`, `implementation` mapping each to a new plain item `login`, as the tools
   `sign-in`, `account`, `sign-out` and `retire`; `records: {inputs: false, outputs: false}`; no
   `offers`. Its description: one account's Codex login on one machine, for its Agents, on any plan,
   signed in and out only by its own hooks.
2. **`codex.v3`**, a new Type after it, family `Agent`: `.v2`'s `schema` without `account`, with
   `login`, required, of schema `github.com/shrav-k/conduit/core#resource-reference.v1`;
   `toolsDirectory: {directory: /directory}`; `loginResource: {place: /login}`; no `signIn`;
   `offers: ['#conversation.v1']`; `requires` `setup-hook.v1` and `retire-hook.v1`; `implementation`
   mapping its four operations to a new item `wrapper-v3`, binding `.v2`'s variables and
   `AGENT_LOGIN_DIRECTORY: /login`, which receives the Resource's directory (PRV-029); no `-path`
   twin, since PRV-029 passes it only on the engine's own runner. The description is `.v2`'s, its
   last two sentences saying the login is the Resource's, linked in, and the retire hook logs
   nothing out. `conduit.yaml` exports both after `- codex.v2`. Every existing document and item
   stays byte for byte (NAM-024).
3. **The Resource's hooks**, moved from `.v2`'s, which stay. The `login` run keeps `codex-home`
   (0700) in its state directory, with `.v2`'s home checks, prefixed `login home (CODEX_HOME):`,
   and runs `.v2`'s hooks there, the file store pinned: `sign-in`, `codex login`; `account`,
   `account/read` with refresh blocked, and where it finds an account but no regular `auth.json`,
   it fails with `login home (CODEX_HOME): the login is not kept in auth.json, so a managed
   setting may have chosen another store; no Agent can share it`. `sign-out` runs `codex logout`
   and answers `client signed out of the login Resource; revocation at the vendor is
   unconfirmed`. `retire` logs out, then empties `codex-home`.
4. **The `.v3` link.** The Agent's own home is `.v2`'s `client-home`, with `.v2`'s checks.
   `AGENT_LOGIN_DIRECTORY/codex-home` is checked (absolute, an existing
   directory inside the variable's, valid Unicode), prefixed `login home
   (AGENT_LOGIN_DIRECTORY):`, never created; its `auth.json`, the target, may be absent. The
   failed-turn causes gain `login-home`, for texts beginning `login home (`.
   - The setup hook, and any launch, makes an absent `client-home/auth.json` a symlink to the
     target, present or not; the hook answers `auth.json linked to the login Resource's file`.
   - Both refuse, replacing nothing and starting no client, a regular file (`client home
     (CODEX_HOME): auth.json is a file, not the login Resource's link; the wrapper replaces no
     login`) and a link elsewhere (`client home (CODEX_HOME): auth.json links somewhere other
     than the login Resource's file`).
   - Each launch adds `use_agent_identity` last to `.v2`'s `--disable`.
5. **The `.v3` retire hook** stops the tree and refuses later turns as `.v2`'s does, runs no
   `codex logout`, removes the link, never its target, empties the rest of the home, and answers
   `client stopped and its home emptied; the login was left to its login Resource`. `.v3` serves
   no sign-out hook: only `sign-in.v1`, the Resource's, calls one (PND-010).
6. **The lapse report** (PND-005, IFC-016), in order 258's form; the item stays plain. A `.v3` turn
   that finds the link dangling, or whose client fails with class `authentication` through `login`,
   `log in`, `credential` or a text below, answers with exactly 258's `input_required` result: one
   URL-mode request `sign-in`, its `message` a fixed text that the login Resource's Codex login is
   missing or lapsed, its `url` `https://learn.chatgpt.com/docs/auth` (OpenAI's sign-in page), no
   other member. No other failure answers so, a usage limit or another `authentication` text (the
   `engine` MCP server's 401) included. A lapse after the client started ends the conversation as a
   failed turn does; a dangling link ends nothing. The engine records the lapse, raises at most one
   sign-in action and one notice on the Resource, holds no Agent, and runs the login's turns one at
   a time until root lifts it; only the Resource's own hold (PND-005, PND-011) holds every Agent.
   The `authentication` class, one table for every version, gains `refresh token`, `refresh_token`,
   `sign in again` and `signed in to another account` (Codex 0.167.1's); `.v1` and `.v2` keep their
   failed result.
7. **0.2.3, for this package alone**, as order 253's decisions 2 and 3 moved 0.2.1 to 0.2.2: the
   version lines and the lockfile's own line; the copy under
   `conformance/fixtures/agents/agent-codex/`, its archive by `make_archives` in write mode, its
   README `(work order 259)`; the Codex pins in `live-agents/fixture.json`,
   `engine/examples/s1/conduit.yaml` and `agents.rs`, whose shared pin line then gives each package
   its own version; and the cases `registration-agent-v2-packages` and
   `registration-agent-v2-account-binding`, the first's Codex install step's `documents` gaining
   `{"kind": "Type", "metadata": {"name": "codex-login.v1"}}` and then `codex.v3`'s, as the file
   writes the others. `make_archives`' 0.2.0 surface leaves out both Types and items, as it leaves
   out the templates; other outputs keep their bytes. Declare it all in
   `docs/work-orders/259-declared.yaml`, as 253's.
8. **Tests**, written first and run on the unchanged code; report each failing line.
   - In `tests/wrapper.rs`: `login_resource_hooks_keep_their_own_home` (decision 3);
     `v3_links_auth_json_and_refuses_others` (decision 4, each refusal leaving the planted file
     and the target unchanged); `v3_retire_never_logs_out_its_login_resource` (no logout
     recorded, the target's bytes and modification time unchanged);
     `v3_lapsed_login_is_reported_as_the_resources` (decision 6's result, member by member, for
     a dangling link and each login text; none for a usage limit or a bare 401; `.v2`'s failed
     result, of class `authentication`);
     `v3_agents_share_only_the_login_file` (two Agents of one Resource; one client saves fake
     tokens as Codex does, a temp file renamed over the target through the link; both links stay
     links and read the new bytes; other entries stay each home's own).
   - In `tests/package.rs`, `codex_login_types_keep_their_shape`: decisions 1 and 2, the logins
     equal to `codex.v2`'s.
   - In `agents.rs`, `codex_login_resource_registers_through_the_engine`, set up as order 253's
     templates test: a project with a Pool holding one `codex-login.v1` Resource (`account:
     account-a`) and two `codex.v3` Agents whose `login` names it is accepted, each login place
     holding the Resource's resolved reference. Today 0.2.2 exports neither Type.
   - Change an existing test only where decisions 1 to 7 require; report each.
9. **The README.**
   - Before `## The standard Conduit templates`: `## The login Resource` (decisions 1 and 3; a
     re-sign-in deletes the old login before the browser step, so a turn meanwhile reports the lapse
     (decision 6); revocation is best effort) and `## The .v3 Agent` (decisions 2, 4 to 6; shared:
     the login file, its lock and temp files, the usage limits; each Agent's: `config.toml`,
     history, sessions, SQLite stores, the context; limits: parallel turns rely on Codex's guards
     and OpenAI's window, a 401-path collision fails one turn, a stop mid-exchange or a sleep past
     expiry can end the login for all, a refresh failure that fails no turn goes unseen, so the
     first one seen may end the login, an Agent's tools can log it out (ruling 70), local disk only,
     Windows links unchecked, `use_agent_identity` read again at each Codex bump, OpenAI's rule from
     its word and the live login, DEC-046).
   - The templates section (order 253's Record): the `#conversation` form written as
     `operator: {from: ...#conversation}`, in `turn`'s words, and a link to the README's
     section on `new-conversation.v1`.
   - Last, `## Moving to 0.2.3`: install the archive, move the pin, replace each Agent, a
     replaced `.v2` Agent signing in again; for `.v3`, retire each `.v2` Agent first (it logs out
     its own login), sign in each account's login Resource once per machine, then register the
     `.v3` Agents.

## What to read

CTR-013, PRV-029, PND-005 and PND-010 as order 257 writes them; order 258 (its form of a plain
turn's lapse, and decisions 1 and 4 to 7) and IFC-016; order 253 and its declaration.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
tr -s ' \n' '  ' < spec/promises/contracts.yaml | grep -oF 'states signIn or loginResource.' | wc -l | tr -d ' '
test "$(git grep -l loginResource -- engine/crates | wc -l)" -gt 0; echo "engine $?"
d=packages/agent-codex; c=conformance/fixtures/agents/agent-codex
echo "$(grep -c '^version: 0\.2\.2$' $d/conduit.yaml) $(grep -c '^version = "0\.2\.2"$' $d/Cargo.toml) $(grep -cE '^- (codex-login\.v1|codex\.v3)$' $d/conduit.yaml) $(grep -c '^kind: Conduit$' $d/contracts.yaml)"
for f in conduit.yaml contracts.yaml; do cmp -s $d/$f $c/$f; echo "$f $?"; done
D=$(shasum -a 256 $c/contracts.tar.gz | cut -c1-64); git grep -l "sha256:$D" | wc -l | tr -d ' '
PYTHONDONTWRITEBYTECODE=1 python3 -c "import json; c=json.load(open('conformance/language/registration/agent-v2-packages/case.yaml')); print(*[len(s['expect']['reply']['package']['project']['documents']) for s in c['steps']])"
git grep -lE 'fn (login_resource_hooks_keep_their_own_home|v3_[a-z_]+|codex_login_[a-z_]+)\(' | wc -l | tr -d ' '
test -e docs/work-orders/259-declared.yaml; echo "declared $?"
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
for p in engine $d; do (cd $p && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "$p $?"; done
(cd engine && cargo run --offline --release -p conduit-conformance --example make_archives -- --check 2>&1 | tail -n 1)
git status --short | wc -l | tr -d ' '
```

They print `1`, `engine 0`, `1 1 0 2`, `conduit.yaml 0`, `contracts.yaml 0`, `7`, `7 7`,
`0`, `declared 1`, two lines ending ` 0`, `N reproducible archive fixtures verified` (keep N) and
`0`.

## Scope

Change, in `packages/agent-codex`, `conduit.yaml`, `contracts.yaml`, `Cargo.toml`, `Cargo.lock`,
`README.md`, `src/` and `tests/`; the copy's `conduit.yaml`, `contracts.yaml`, `contracts.tar.gz`
and `README.md`; `live-agents/fixture.json`; the two cases' Codex install files, the binding
case's `conduit.json` and each `case.yaml`; `engine/examples/s1/conduit.yaml`; `agents.rs`;
`make_archives.rs`; and the new `docs/work-orders/259-declared.yaml`. Change nothing else: not
`spec/`, `packages/agent-claude-code` or its copy, another case or fixture, an engine source,
`experiments/` or a dependency; report what else needs it.

## Done when

Each line of the first group prints otherwise on `main` after order 258 ("today") and passes
after:

```sh
ok='^test result: ok\. 1 passed'
d=packages/agent-codex; c=conformance/fixtures/agents/agent-codex
echo "$(grep -c '^version: 0\.2\.3$' $d/conduit.yaml) $(grep -c '^version = "0\.2\.3"$' $d/Cargo.toml) $(grep -cE '^- (codex-login\.v1|codex\.v3)$' $d/conduit.yaml) $(grep -cE '^  name: (codex-login\.v1|codex\.v3)$' $d/contracts.yaml) $(grep -cE '^## (The login Resource|The \.v3 Agent|Moving to 0\.2\.3)$' $d/README.md)"  # 1 1 2 2 3; today 0 0 0 0 0
echo "$(grep -c 'operator: {from: example.org/conduit/agent-codex#conversation}' $d/README.md) $(sed -n '/^## The standard Conduit templates$/,/^## M/p' $d/README.md | grep -c '(#a-new-conversation-both-versions)')"  # 1 1; today 0 0
for t in login_resource_hooks_keep_their_own_home v3_links_auth_json_and_refuses_others v3_retire_never_logs_out_its_login_resource v3_lapsed_login_is_reported_as_the_resources v3_agents_share_only_the_login_file; do (cd $d && CARGO_NET_OFFLINE=true cargo test --offline --release --test wrapper $t 2>&1 | grep -c "$ok"); done  # 1 each; today 0
(cd $d && CARGO_NET_OFFLINE=true cargo test --offline --release --test package codex_login_types_keep_their_shape 2>&1 | grep -c "$ok")  # 1; today 0
(cd engine && CARGO_NET_OFFLINE=true cargo test --offline --release -p conduit-shell --test agents codex_login_resource_registers_through_the_engine 2>&1 | grep -c "$ok")  # 1; today 0
PYTHONDONTWRITEBYTECODE=1 python3 -c "import json; c=json.load(open('conformance/language/registration/agent-v2-packages/case.yaml')); print(*[len(s['expect']['reply']['package']['project']['documents']) for s in c['steps']])"  # 7 9; today 7 7
echo "$(grep -c 'at version 0\.2\.3 (work order 259)' $c/README.md) $(grep -c 'version: 0\.2\.3' engine/examples/s1/conduit.yaml)"  # 1 1; today 0 0
r=; for i in 1 2; do r="$r $(cd $d && CARGO_NET_OFFLINE=true cargo run --offline --locked --release --bin package 2>/dev/null | sed 's|^.*/dist/|dist/|')"; done; echo "$r"; tar -xOzf $d/dist/agent-codex.tar.gz conduit.yaml | grep -c '^version: 0\.2\.3$'  # one dist/agent-codex.tar.gz#SHA256 twice, then 1; today the 1 is 0
out=$(/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/259-declared.yaml); rc=$?; printf '%s\n' "$out" | tail -n 2; echo "exit $rc"  # 0 undeclared, 0 errors; exit 0; today exit 2
```

Guards, passing today and after; the two suites run once each, last:

```sh
d=packages/agent-codex; c=conformance/fixtures/agents/agent-codex
for f in conduit.yaml contracts.yaml; do cmp -s $d/$f $c/$f; echo "$f $?"; tar -xOzf $c/contracts.tar.gz $f | cmp -s - $d/$f; echo "archive $f $?"; done; D=$(shasum -a 256 $c/contracts.tar.gz | cut -c1-64); git grep -l "sha256:$D" | wc -l | tr -d ' '  # each 0, then 7
(cd engine && cargo run --offline --release -p conduit-conformance --example make_archives -- --check 2>&1 | tail -n 1)  # the start check's line
git diff --numstat HEAD -- $d/Cargo.lock | cut -f1,2  # 1 1
out=$(/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared-changed); rc=$?; printf '%s\n' "$out" | tail -n 1; echo "exit $rc"  # as above; exit 0
(cd $d && cargo fmt --check && CARGO_NET_OFFLINE=true cargo clippy --offline --all-targets -- -D warnings)
(cd engine && cargo fmt --all -- --check && cargo clippy --offline -p conduit-shell -p conduit-conformance --all-targets -- -D warnings)
for t in check_decisions.py\ HEAD\ WORKTREE check_docs.py promise_standing.py\ --check check_private_paths.py; do PYTHONDONTWRITEBYTECODE=1 python3 tools/$t | tail -1; done  # Errors: 0 each
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(packages/agent-codex/(Cargo\.(toml|lock)|[a-z]+\.yaml|README\.md|src/.+|tests/.+)|conformance/fixtures/agents/agent-codex/[a-zA-Z.]+|conformance/language/registration/(live-agents|agent-v2-[a-z-]+)/[a-z-]+\.(json|yaml)|engine/examples/s1/conduit\.yaml|engine/crates/(shell/tests/agents|conformance/examples/make_archives)\.rs|docs/work-orders/259-declared\.yaml)$' | wc -l | tr -d ' '  # 0
(cd $d && CARGO_NET_OFFLINE=true cargo test --offline --release)  # every test passes
(cd engine && CARGO_NET_OFFLINE=true RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release)  # the whole engine suite, every vector among it
```

Report, per decision, what changed; each new test's failing, then passing line; both
`dist/agent-codex.tar.gz#SHA256` lines with `rustc -vV`'s `host:` and `release:`; and every check's
last line. Stage nothing.

## What follows

- The fake-token probe of Codex's client side (the study's §8) runs the real Codex, which no dogfood
  test may: its own order on the Linux node, before the sitting, launching `codex exec --disable
  use_agent_identity` once, as each `.v3` launch keeps `--strict-config`.
- The second-account sitting (DEC-042), README's steps, and the live watch (DEC-046).

## Record

Run on 2026-11-22 through Conduit itself, by the dogfood's own Agents, in Pacific time, on
`main` with orders 257 and 258 stacked while they waited in the merge queue; it follows them as
built, with the coordinator's amendments during their builds (open to the owner's strike).

**Worker-1's turn** (02:35 to 03:00) released agent-codex 0.2.3: a `codex-login.v1` Resource Type
keeping one account's `codex-home`, with the sign-in, account, sign-out and retire hooks, and a
`codex.v3` Agent Type whose own home links only `auth.json` to the Resource's file, made and
checked at setup and every launch, removed at retirement, the target kept; a lapse reported in
IFC-016's URL form; `use_agent_identity` off. Seven regressions failed first and pass; 45
package tests and three vectors pass; two offline builds give one archive digest.

**Two blind reads at once** (03:00 to 03:05), the owner's answer for A77's window:
`turn-verifier-1.turn.v1` found two musts and confirmed that the library's empty
`elicitationId` is harmless, the engine reading the result's type, method and mode; the Claude
reader passed it with one should and two nits. One fix turn (03:05 to 03:10), each must reproduced
first:
- a link ending in `auth.json/.` passed setup though it reaches no file; the raw last component is
  now checked;
- an engine MCP 401 with login wording could read as the shared login's lapse; the engine's MCP
  failures are now excluded from that branch;
- the README gains a worked setup (a Resource in a Pool, a `.v3` Agent on its node, the Resource
  signed in first, and moving from `.v2`), and a missing login home says to set up or sign in the
  Resource first.

The gate's run on the squash is the coordinator's, at the merge.
