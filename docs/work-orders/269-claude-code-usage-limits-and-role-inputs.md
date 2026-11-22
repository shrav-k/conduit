# Work order 269: Claude Code turns report their usage (IFC-051) and a usage limit's window and reset, and `claude-code.v4` takes role instructions and an output schema, released as 0.2.4

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`: list in
`vectors` each case decision 7 changes, its promises and whether it passes; add no case. It needs
no network; its tests bind only loopback ports. Run cargo offline, with `CARGO_NET_OFFLINE=true` on
each command that builds a package, and fetch nothing. Run one cargo command at a time. Never print
a whole large file, case, diff or log. Never run `env`, `printenv`, `set` or `export -p`, never
print an environment variable's value, and never read or print a key, a token or any credential, a
test's fake one included. Run no `conduit` outside a test, no real `claude` or `codex`, no dogfood
harness and not `tools/gate.py`, and sign in nowhere: tests use the scripted client, which
reproduces the outputs Anthropic documents, and fake values only. Prefix each `python3` with
`PYTHONDONTWRITEBYTECODE=1`. Every file you write is public, your report included: no absolute
local path, user, machine or account name; write an archive as `dist/NAME.tar.gz#SHA256`.

## What and why

**The owner's answers**, as [order 260](260-codex-usage-limits-and-role-inputs.md) gives them:
[DEC-046](../decisions/log.md#dec-046-2026-11-21-the-fleet-grows-on-every-account-by-measurement)'s
"Pause that account (Recommended)", and
[DEC-047](../decisions/log.md#dec-047-2026-11-21-usage-in-records-and-harnesses-in-role-templates)'s
"Engine keeps them now (Recommended)" and "Role templates + inputs (Recommended)". Order 260 did
Codex's half, in the shapes this order repeats; the two did not fit one worker turn and 20 KB.

**Today** (0.2.3, order 264). The class table misses Claude Code's own limit words: stage 2's turn
89, `reader-1`'s, failed as `other` while the coordinator's session on the same Mac printed
"You've hit your weekly limit · resets 1am (America/Los_Angeles)" (whether it shares `reader-1`'s
account, and what `reader-1`'s client printed, is not shown), and only a restart of serve
recovered
([the results](../../experiments/028-a78-dogfood/results-2026-11-21-stage-2.md#turns-lost-and-other-failures)).
No result reports tokens.

**Anthropic's words** (read 2026-11-22):

- The errors page: "You've hit your session limit · resets 3:45pm", and the same for `weekly`,
  `Opus` and `Sonnet`; "Claude Code blocks further requests until the reset time shown".
- The Agent SDK reference, whose messages are `stream-json`'s: `rate_limit_event`, "Emitted when the
  session encounters a rate limit", with `rate_limit_info.status` `"allowed" | "allowed_warning" |
  "rejected"` and `resetsAt?: number`, no unit stated and no window; `errorCode:
  "credits_required"` is "a claude.ai subscription whose included usage is exhausted".
- `result` holds `modelUsage`, "per-model totals ... including the main loop, subagents, and
  internal calls"; "In streaming-input sessions the totals are cumulative across turns, so read the
  latest result rather than summing across results". `ModelUsage` has `inputTokens`,
  `outputTokens`, `thinkingTokens` ("`outputTokens` already includes them"),
  `cacheReadInputTokens` and `cacheCreationInputTokens`; the prompt-caching page: "total_input_tokens
  = cache_read_input_tokens + cache_creation_input_tokens + input_tokens". The error arm carries
  `modelUsage` too. `system/init` holds `claude_code_version` and `model`.
- `--append-system-prompt-file`: "Load additional system prompt text from a file and append to the
  default prompt". `--json-schema`: "Get validated JSON output matching a JSON Schema after the
  agent completes its workflow (print mode only)", in `structured_output`; "If a run ends without
  valid structured output ... the response has no `structured_output` field"; the error subtype
  `error_max_structured_output_retries`.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 4, both
packages give one shape (`usage.v1`, `limit`, the two places, `structured`); 2, a role's text and
schema are inspectable config; 6, nothing is read or retried at a limit. None at risk: the
instructions file is the run's own, never in the shared account home.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes,
by existing practice: each is the client's own flag, message or count. Not taken: the reset from
the message's local time, which names no date; the home's `stats-cache.json` (the account's
totals); `result`'s `usage` ("main agent loop only ... Prefer `modelUsage`"); places on `.v2` and
`.v3` (NAM-024, MAN-017, as order 260 says).

**When it runs.** On `main` once order 260 merges: both move the same pins. Through Conduit's
dogfood: one Codex worker turn (a 4-hour limit); a Codex verifier and a Claude reader, blind and at
once; one fix turn; the gate (DEC-046). By ruling 89, how is yours. Names, texts, the window words,
the two readings of `resetsAt` and the subtraction are the coordinator's, open to the owner's
strike.

**Stop rule.** Stop, change nothing further, and report where the engine refuses
`claude-code.v4`; a correct result needs a change outside scope; `vector_delta.py` reports an
undeclared change or a vector fails; two `package` runs differ; a test would start a real client;
or a crate is missing. A choice no decision governs is yours: list it under `own_choices`.

## The decisions

1. **`claude-code.v4`**, a new Type after `claude-code.v3`, as order 260's decision 1 makes
   `codex.v4` from `codex.v3`: the same two places, a new item `wrapper-v4`, `wrapper-v3`'s
   bindings plus `AGENT_INSTRUCTIONS: /instructions` and `AGENT_OUTPUT_SCHEMA: /outputSchema`, and
   `.v3`'s description with two sentences: `instructions` is appended to the client's system
   prompt; `outputSchema`, the JSON text of a JSON Schema whose `type` is `object`, is the schema
   the client validates its final output against, which the turn returns in `structured`.
   `conduit.yaml` exports it after `- claude-code.v3`. Every other document and item stays byte
   for byte, but decision 2's line (NAM-024).
2. **`turn.v1`'s `structured`**: order 260's decision 2, the same line in the same place.
3. **Instructions and schema**, read wherever the variables are present (only `wrapper-v4` binds
   them, so `.v4` with both null launches as `.v3`):
   - The wrapper writes the text to `instructions.md` in the run's private directory, beside
     `settings.json`, and the launch adds `--append-system-prompt-file` with its path.
   - The schema is checked as order 260 checks it, with its refusal text; the launch adds
     `--json-schema` and the text.
   - A `result` whose `structured_output` is an object returns it as `structured`, `text`
     unchanged; otherwise none. A `result` whose `subtype` is
     `error_max_structured_output_retries` fails the turn with class `structured-output`, a new
     row before `other`, matched by the subtype and not by a substring.
4. **Model, effort and what stays off.** `model` on every version and `effort` on `.v2` and later
   stay config places, default `''`, mapped to `--model` and `--effort` as today. Auto memory is
   off (order 264's decision 5); hooks (`disableAllHooks`), plugins (`enabledPlugins: {}`), the
   sub-agent tools (not in `--tools`), slash commands and skills (`--disable-slash-commands`) and
   setting sources (`--setting-sources ""`) stay as today. No flag is added. The CLI reference
   names only user, project and local settings for `--setting-sources`, so the README's table
   marks the project's CLAUDE.md as unchecked until the first real turns.
5. **The usage member** (IFC-051), on every turn that ends at a `result`, failed or not, of any
   version: `_meta`'s `io.github.shrav-k.conduit/usage`,
   `{"client":{"name":"claude-code","version":V},"models":{M:{"input":I,"cachedInput":C,"output":O,"reasoning":R}}}`.
   - M is each `modelUsage` key, as the client gives it. I is `inputTokens` +
     `cacheReadInputTokens` + `cacheCreationInputTokens`, C `cacheReadInputTokens`, O
     `outputTokens`, R `thinkingTokens`, left out where absent: `usage.v1`'s meanings.
   - Each count less the same model's at the same client process's previous `result`; a model
     absent there, as reported; a model whose differences are all 0 is left out.
   - V is the latest `system/init`'s `claude_code_version` in that process, matching
     `^[0-9A-Za-z.+-]{1,64}$`.
   - No member where V is unread, `modelUsage` is absent, a count fell, no model is left, or more
     than 16 are (`usage.v1`'s bound). Nothing else in a result changes; order 262's engine
     keeps the member.
6. **A limit's window and reset** (DEC-046), in order 260's `limit` (`.v2` and later):
   - The `usage-limit` row gains `hit your session limit`, `hit your weekly limit`, `hit your opus
     limit` and `hit your sonnet limit`, which match after a straight or a curly apostrophe. A turn
     that fails as `client failure` after its stream held a `rate_limit_event` with status
     `rejected` has class `usage-limit`, whatever its text: the structured signal wins. The event
     fails no turn: one whose `result` is not an error completes.
   - `window` is `session`, `weekly`, `opus` or `sonnet`, from the matched words, else null.
     `windowMinutes` is null: Claude Code states no length.
   - `resetsAt` comes from the turn's last `rejected` event. Its unit is unstated, so the wrapper
     reads the number as Unix seconds and as milliseconds and takes the one reading that falls
     after the turn began and within 8 days of it, since a week's window resets inside that, in
     RFC 3339 UTC. Neither, or no such event: null. Never from the text.
   - An `allowed_warning` event changes nothing.
7. **0.2.4, for this package alone**, as order 260's decision 7 moved `agent-codex`: the version
   lines and the lockfile's own line; the copy under
   `conformance/fixtures/agents/agent-claude-code/`, its archive by `make_archives` in write mode,
   its README `(work order 269)`; the Claude Code pins in `live-agents/fixture.json`,
   `engine/examples/s1/conduit.yaml` and `agents.rs`; and the two `registration-agent-v2-*` cases,
   the Claude Code install step's `documents` gaining `{"kind": "Type", "metadata": {"name":
   "claude-code.v4"}}` after `claude-code.v3`'s. `make_archives`' 0.2.0 surface leaves out
   `claude-code.v4`, `wrapper-v4` and `structured`; other outputs keep their bytes. Declare it all
   in `docs/work-orders/269-declared.yaml`, as 260's.
8. **Tests**, written first and run on the unchanged code; report each failing line.
   - In `tests/wrapper.rs`: `claude_turns_report_their_usage` (decision 5: a first `result` with
     two models; a second's cumulative totals giving differences, an unchanged model left out; a
     failed `result`'s member; a new conversation restarting the baseline; none for no version, no
     `modelUsage` and a fall); `usage_limit_reports_the_claude_window_and_reset` (decision 6: the
     four texts with both apostrophes, and the weekly one with stage 2's `resets 1am
     (America/Los_Angeles)` as Anthropic's documented wording, not as `reader-1`'s output, `other`
     today, giving class and window; a `rejected` event an hour ahead in seconds and in
     milliseconds giving one time; 30 days ahead or past giving null; a failed turn's `rejected`
     event with other text giving `usage-limit`, window null; a `rejected` event before a `result`
     that is not an error giving a completed turn; `allowed_warning` alone giving nothing; `limit:
     null` for other classes);
     `v4_maps_instructions_and_output_schema` (decisions 3 and 4: both flags, the file's bytes,
     outside the account home, `structured` or none, `structured-output`, the refusal starting no
     client, `.v3`'s launch unchanged).
   - In `tests/package.rs`, `claude_code_v4_keeps_its_shape`: decisions 1 and 2.
   - In `agents.rs`, `claude_code_v4_registers_through_the_engine`, as 264's login test: a Pool's
     `account-home.v1` Resource and two `claude-code.v4` Agents, one setting both places, one
     leaving them null, accepted as set.
   - Change an existing test only where decisions 1 to 7 require.
9. **The README**, as order 260's decision 9: before `## The standard Conduit templates`, `## The
   .v4 Agent`, `## Usage in each turn's result`, `## A usage limit's window and reset` and `##
   Model, effort and what stays off`; `limit` and the new class rows in the failed turn's tables;
   last, `## Moving to 0.2.4`.

## What to read

Order 260 and its declaration; IFC-051, `usage.v1`, NAM-024, MAN-017; orders 264 and 261; this
package; `agents.rs`.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
grep -cE '^(version: 0\.2\.4|- codex\.v4)$' packages/agent-codex/conduit.yaml
d=packages/agent-claude-code; c=conformance/fixtures/agents/agent-claude-code
echo "$(grep -c '^version: 0\.2\.3$' $d/conduit.yaml) $(grep -c '^version = "0\.2\.3"$' $d/Cargo.toml) $(grep -cE '^- claude-code\.v[34]$' $d/conduit.yaml) $(grep -c '^      structured: {type: object}$' $d/contracts.yaml)"
for f in conduit.yaml contracts.yaml; do cmp -s $d/$f $c/$f; echo "$f $?"; done
D=$(shasum -a 256 $c/contracts.tar.gz | cut -c1-64); git grep -l "sha256:$D" | wc -l | tr -d ' '
PYTHONDONTWRITEBYTECODE=1 python3 -c "import json; c=json.load(open('conformance/language/registration/agent-v2-packages/case.yaml')); print(*[len(s['expect']['reply']['package']['project']['documents']) for s in c['steps']])"
git grep -l -e 'conduit/usage' -e 'hit your weekly limit' -- $d | wc -l | tr -d ' '
git grep -lE 'fn (claude_turns_report_their_usage|usage_limit_reports_the_claude_window_and_reset|claude_code_v4_[a-z_]+)\(' | wc -l | tr -d ' '
test -e docs/work-orders/269-declared.yaml; echo "declared $?"
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
for p in engine $d; do (cd $p && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "$p $?"; done
(cd engine && cargo run --offline --release -p conduit-conformance --example make_archives -- --check 2>&1 | tail -n 1)
git status --short | wc -l | tr -d ' '
```

They print `2`, `1 1 1 0`, `conduit.yaml 0`, `contracts.yaml 0`, `7`, `9 10`, `0`, `0`,
`declared 1`, two lines ending ` 0`, `N reproducible archive fixtures verified` (keep N) and `0`.

## Scope

Change, in `packages/agent-claude-code`, `conduit.yaml`, `contracts.yaml`, `Cargo.toml`,
`Cargo.lock`, `README.md`, `src/` and `tests/`; the copy's `conduit.yaml`, `contracts.yaml`,
`contracts.tar.gz` and `README.md`; `live-agents/fixture.json`; the two cases' Claude Code install
files, the binding case's `conduit.json` and each `case.yaml`; `engine/examples/s1/conduit.yaml`;
`agents.rs`; `make_archives.rs`; and the new `docs/work-orders/269-declared.yaml`. Change nothing
else: not `spec/`, `packages/agent-codex` or its copy, another case or fixture, an engine source,
`experiments/` or a dependency; report what else needs it.

## Done when

Each line of the first group prints otherwise on `main` after order 260 ("today") and passes
after:

```sh
ok='^test result: ok\. 1 passed'
d=packages/agent-claude-code; c=conformance/fixtures/agents/agent-claude-code
echo "$(grep -c '^version: 0\.2\.4$' $d/conduit.yaml) $(grep -c '^version = "0\.2\.4"$' $d/Cargo.toml) $(grep -c '^- claude-code\.v4$' $d/conduit.yaml) $(grep -c '^  name: claude-code\.v4$' $d/contracts.yaml) $(grep -c '^      structured: {type: object}$' $d/contracts.yaml) $(grep -cE '^## (The \.v4 Agent|Usage in each turn.s result|A usage limit.s window and reset|Model, effort and what stays off|Moving to 0\.2\.4)$' $d/README.md)"  # 1 1 1 1 1 5; today 0 0 0 0 0 0
for t in claude_turns_report_their_usage usage_limit_reports_the_claude_window_and_reset v4_maps_instructions_and_output_schema; do (cd $d && CARGO_NET_OFFLINE=true cargo test --offline --release --test wrapper $t 2>&1 | grep -c "$ok"); done  # 1 each; today 0
(cd $d && CARGO_NET_OFFLINE=true cargo test --offline --release --test package claude_code_v4_keeps_its_shape 2>&1 | grep -c "$ok")  # 1; today 0
(cd engine && CARGO_NET_OFFLINE=true cargo test --offline --release -p conduit-shell --test agents claude_code_v4_registers_through_the_engine 2>&1 | grep -c "$ok")  # 1; today 0
PYTHONDONTWRITEBYTECODE=1 python3 -c "import json; c=json.load(open('conformance/language/registration/agent-v2-packages/case.yaml')); print(*[len(s['expect']['reply']['package']['project']['documents']) for s in c['steps']])"  # 10 10; today 9 10
echo "$(grep -c 'at version 0\.2\.4 (work order 269)' $c/README.md) $(grep -c 'version: 0\.2\.4' engine/examples/s1/conduit.yaml)"  # 1 2; today 0 1
r=; for i in 1 2; do r="$r $(cd $d && CARGO_NET_OFFLINE=true cargo run --offline --locked --release --bin package 2>/dev/null | sed 's|^.*/dist/|dist/|')"; done; echo "$r"; tar -xOzf $d/dist/agent-claude-code.tar.gz conduit.yaml | grep -c '^version: 0\.2\.4$'  # one dist/agent-claude-code.tar.gz#SHA256 twice, then 1; today the 1 is 0
out=$(/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/269-declared.yaml); rc=$?; printf '%s\n' "$out" | tail -n 2; echo "exit $rc"  # 0 undeclared, 0 errors; exit 0; today exit 2
```

Guards, passing today and after; the two suites run once each, last:

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
git status --short -uall | sed 's/^...//' | grep -vE '^(packages/agent-claude-code/(Cargo\.(toml|lock)|[a-z]+\.yaml|README\.md|src/.+|tests/.+)|conformance/fixtures/agents/agent-claude-code/[a-zA-Z.]+|conformance/language/registration/(live-agents|agent-v2-[a-z-]+)/[a-z-]+\.(json|yaml)|engine/examples/s1/conduit\.yaml|engine/crates/(shell/tests/agents|conformance/examples/make_archives)\.rs|docs/work-orders/269-declared\.yaml)$' | wc -l | tr -d ' '  # 0
(cd $d && CARGO_NET_OFFLINE=true cargo test --offline --release)  # every test passes
(cd engine && CARGO_NET_OFFLINE=true RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release)  # the whole engine suite, every vector among it
```

Report per decision what changed; each new test's failing, then passing line; both
`dist/agent-claude-code.tar.gz#SHA256` lines with `rustc -vV`'s `host:` and `release:`; each
existing test changed, and why; every check's last line. Stage nothing.

## What follows

- The role templates (DEC-047): readers on `claude-code.v4` with `model`, `effort`,
  `instructions` and `outputSchema`; and the pause until `resetsAt` (DEC-046), by order.
- The first real turns after the sitting check what no scripted client can: `--json-schema`
  under `--input-format stream-json`, a later `result`'s cumulative `modelUsage`, a
  `rejected` event's `resetsAt` against the message's time, and no project CLAUDE.md in the
  turn's context under `--setting-sources ""`. Each goes in a Record.

## Record

Run on 2026-11-22 through Conduit itself, by the dogfood's own Agents, in Pacific time, on
`main` with orders 258, 259, 264 and 260 stacked while they waited in the merge queue; it carries
the rules order 260's fix turn settled (the coordinator's, open to the owner's strike).

**Worker-1's turn** (04:24 to 04:40) released agent-claude-code 0.2.4: `claude-code.v4` with
optional `instructions`, passed by a run-local file written without following links through
`--append-system-prompt-file`, and `outputSchema` through `--json-schema`, each limited to fit
Linux's per-string limit; each turn's usage as IFC-051's member from `modelUsage`'s cumulative
counts, cache reads and writes inside `input`; the four limit phrases and the rejected event
giving `usage-limit` with its window and reset. 51 package tests and three vectors pass.

**Two blind reads at once** (04:41 to 04:44): `turn-verifier-1.turn.v1` found three musts; the
Claude reader passed it. One fix turn (04:45 to 04:49), each must reproduced first, each by the
rule never to report a count that cannot be measured:
- a cache counter first seen on a resumed turn inflated `input`; a model's report is suppressed
  for a turn whose components are not comparable with the previous report;
- a missing or unreadable report cleared the baseline, so the next one re-reported cumulative
  counts; the baseline is now re-established without attributing them to one turn;
- the limit phrases inside the SDK error result's `errors` were missed; they are now read too,
  keeping raw client text out of the reply.

54 package tests pass; the archive reproduces. The gate's run on the squash is the
coordinator's, at the merge.
