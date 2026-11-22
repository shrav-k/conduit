# Work order 260: Codex turns report their usage (IFC-051) and a usage limit's window and reset, and `codex.v4` takes role instructions and an output schema, released as 0.2.4

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`: list in
`vectors` each case decision 7 changes, its promises and whether it passes; add no case. It needs
no network; its tests bind only loopback ports. Run cargo offline, with `CARGO_NET_OFFLINE=true` on
each command that builds a package, and fetch nothing. Run one cargo command at a time. Never print
a whole large file, case, diff or log. Never run `env`, `printenv`, `set` or `export -p`, never
print an environment variable's value, and never read or print a key, a token or any credential, a
test's fake one included. Run no `conduit` outside a test, no real `codex` or `claude`, no dogfood
harness and not `tools/gate.py`, and sign in nowhere: tests use the scripted client, which
reproduces the outputs OpenAI documents, and fake values only. Prefix each `python3` with
`PYTHONDONTWRITEBYTECODE=1`. Every file you write is public, your report included: no absolute
local path, user, machine or account name; write an archive as `dist/NAME.tar.gz#SHA256`.

## What and why

**The owner's answers** (2026-11-21). [DEC-046](../decisions/log.md#dec-046-2026-11-21-the-fleet-grows-on-every-account-by-measurement):
"Pause that account (Recommended)": a turn that ends on a usage limit reports class
`usage-limit`, the window and the reset time, from the client's structured signal where one
exists, so the coordinator pauses only that account's lanes until the reset.
[DEC-047](../decisions/log.md#dec-047-2026-11-21-usage-in-records-and-harnesses-in-role-templates):
"Engine keeps them now (Recommended)", each turn's result carrying the tokens its client reported
(IFC-051, `usage.v1`, orders 261 and 262); and "Role templates + inputs (Recommended)", optional
`instructions` and `outputSchema` mapped to each client's flags. This order is Codex's half; order
269 is Claude Code's. Today (0.2.3) no result reports tokens, a limit fails as `client failure:
usage-limit` alone, and role text and report schemas ride in each message.

**OpenAI's words** (read 2026-11-22; Codex `rust-v0.167.1`'s source where a page is silent):

- The non-interactive page: `turn.completed` carries `usage` with `input_tokens`,
  `cached_input_tokens`, `output_tokens` and `reasoning_output_tokens`; its example has 24448
  cached of 24763 input. The source fills them from the thread's running total
  (`usage_from_last_total`), with zeros when no count arrived, and a resume seeds that total from
  the rollout (openai/codex pull 18023). `turn.failed` and `error` carry `message` alone; no exec
  event names the model or a limit.
- `--output-schema`: "Path to a JSON Schema for the model's final response" (`global = true`, so
  accepted after `resume`). `developer_instructions`: "Additional developer instructions injected
  into the session". `features.memories`: "Enable Memories (off by default)". Codex scans
  `.agents/skills` "from your current working directory up to the repository root", and
  `$HOME`'s; `skills.include_instructions`: "Include the automatic skills instructions in the
  model context. Defaults to `true`".
- The app-server's `account/rateLimits/read`: `rateLimits` is "the backward-compatible
  single-bucket view", `rateLimitsByLimitId` "the multi-bucket view"; in each, `primary` and
  `secondary` give `usedPercent`, `windowDurationMins` and `resetsAt` ("a Unix timestamp
  (seconds) for the next reset"), and `rateLimitReachedType`, "the server-classified limit state",
  its values unlisted: Codex's one structured signal.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 4, both
packages give one shape (`usage.v1`, `limit`, the two places); 2, a role's text and schema are
inspectable config; 6, the limit read starts no model work. At risk, 3: the read uses the shared
login, refresh blocked.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes,
by existing practice: each is the client's own flag, count or method. Not taken: the session
files under the Codex home (undocumented); the reset from the message's local time; places on
`.v2` and `.v3`, since NAM-024 adds only an optional property with no `default` and MAN-017 binds
only a required scalar place, so a `.v4` is the least; a CTR-006 control for model or effort: a
role template fixes both (DEC-047), and replacing an Agent costs no sign-in; a `turn.v1` input
for instructions, since Claude Code fixes its system prompt when its client starts.

**When it runs.** On `main` once orders 259 and 264 merge; order 269 follows (the same pins).
Through Conduit's dogfood (DEC-046): one Codex worker turn (4 hours), a Codex verifier and a
Claude reader blind and at once, one fix turn, the gate. By ruling 89, how is yours. Names,
texts, `limit`'s shape, the buckets, `.v4`, JSON text, the subtraction and the model key are the
coordinator's, open to the owner's strike.

**Stop rule.** Stop, change nothing further, and report where the engine refuses `codex.v4`; a
correct result needs a change outside scope; `vector_delta.py` reports an undeclared change or
a vector fails; two `package` runs differ; a test would start a real client; or a crate is
missing. A choice no decision governs is yours: list it under `own_choices`.

## The decisions

1. **`codex.v4`**, a new Type after `codex.v3`, family `Agent`: `.v3`'s document, with `schema`
   gaining `instructions` and `outputSchema`, each `{type: [string, 'null'], minLength: 1,
   maxLength: 65536, default: null}` and listed in `required` as `.v3`'s other places are, and its
   operations and hooks served by a new item `wrapper-v4`, `wrapper-v3`'s bindings plus
   `AGENT_INSTRUCTIONS: /instructions` and `AGENT_OUTPUT_SCHEMA: /outputSchema`. Its description is
   `.v3`'s and two sentences: `instructions` is added to the client's own as developer
   instructions; `outputSchema`, the JSON text of a JSON Schema whose `type` is `object`,
   constrains the final response, which the turn returns parsed in `structured`. `conduit.yaml`
   exports it after `- codex.v3`. Every other document and item stays byte for byte, but
   decision 2's line (NAM-024).
2. **`turn.v1`'s `structured`.** Its `outputSchema`'s `properties` gains, after `clientTools`,
   `structured: {type: object}`, not in `required`: an optional addition (NAM-024).
3. **Instructions and schema**, read wherever the variables are present (only `wrapper-v4` binds
   them; a null place binds none, so `.v4` with both null launches as `.v3`):
   - Each launch, first and resumed, adds `-c developer_instructions=` and the text as a TOML
     string, as the wrapper writes its other `-c` strings.
   - `AGENT_OUTPUT_SCHEMA` must parse as a JSON object whose `type` is `"object"`; else every turn
     reports `invalid configuration: outputSchema (AGENT_OUTPUT_SCHEMA): expected the JSON
     text of a schema whose type is object` and starts no client. The wrapper writes it, byte for
     byte, to `output-schema.json` in the run's private directory, and each launch adds
     `--output-schema` with that path.
   - A completed turn whose final `agent_message` text parses as a JSON object returns it as
     `structured`, `text` unchanged; any other, none. The wrapper does not check it against the
     schema: Codex's model service enforces it.
4. **Model, effort and what stays off.** `model` on every version and `effort` on `.v2` and
   later stay config places, default `''` (the client's own), mapped to `--model` and `-c
   model_reasoning_effort=`. Every launch of every version adds `memories` to its `--disable` list
   and `-c skills.include_instructions=false`: the skills catalog is repository discovery, off by
   DEC-047. Hooks, plugins, the multi-agent tools, rules and AGENTS.md stay off as today.
5. **The usage member** (IFC-051), on a completed turn of any version: the result's `_meta` holds
   `io.github.shrav-k.conduit/usage` in `usage.v1`'s shape,
   `{"client":{"name":"codex","version":V},"models":{M:{"input":I,"cachedInput":C,"output":O,"reasoning":R}}}`.
   - I, C, O and R are `turn.completed`'s `input_tokens`, `cached_input_tokens`, `output_tokens`
     and `reasoning_output_tokens`, less the same counts at the conversation's last completed turn
     that reported any; a thread's first turn's as reported. OpenAI's input counts cached tokens
     and its output reasoning ones, as `usage.v1`'s do. C and R are left out where Codex gives
     none.
   - M is the configured `model`: no exec event names one.
   - V is the word after `codex-cli ` that `codex --version` prints, read once per run within 5
     seconds, matching `^[0-9A-Za-z.+-]{1,64}$`.
   - No member where `model` is `''`, V is unread, `usage` is absent, the difference's input is 0
     (Codex's zeros), a count fell, or the turn failed (`turn.failed` carries no counts). Nothing
     else in a result changes.
6. **A limit's window and reset** (DEC-046). The `failedTurn` object (`.v2` and later) gains
   `limit`: `null` unless `class` is `usage-limit`, then
   `{"window": null, "windowMinutes": N, "resetsAt": T}`.
   - After the stop, the wrapper asks `account/rateLimits/read` through `codex app-server`, as
     the account hook asks `account/read`, with `REFRESH_BLOCKED`, within 15 seconds and before
     the turn's deadline. Of the windows of `rateLimits` and of each `rateLimitsByLimitId` entry,
     it takes those at 100 `usedPercent` or more, else the fuller window of each with a non-null
     `rateLimitReachedType`; of those, the latest `resetsAt`: N is its `windowDurationMins` and T
     its `resetsAt` in RFC 3339 UTC, as `2024-11-07T02:55:00Z`, each null where absent. Where none
     is taken, the read fails or no answer comes, N and T are null.
   - `window` is null: Codex names no window (order 269 fills it). `limit` holds fixed words and
     numbers, never client text.
7. **0.2.4, for this package alone**, as order 259's decision 7 moved it to 0.2.3: the version
   lines and the lockfile's own line; the copy under `conformance/fixtures/agents/agent-codex/`,
   its archive by `make_archives` in write mode, its README `(work order 260)`; the Codex pins in
   `live-agents/fixture.json`, `engine/examples/s1/conduit.yaml` and `agents.rs`; and the two
   `registration-agent-v2-*` cases, the packages case's Codex install step's `documents` gaining
   `{"kind": "Type", "metadata": {"name": "codex.v4"}}` after `codex.v3`'s. `make_archives`' 0.2.0 surface leaves out `codex.v4`, `wrapper-v4` and
   `structured`; other outputs keep their bytes. Declare it all in
   `docs/work-orders/260-declared.yaml`, as 259's.
8. **Tests**, written first and run on the unchanged code; report each failing line.
   - In `tests/wrapper.rs`: `codex_turns_report_their_usage` (decision 5: the page's example on a
     first turn; a resumed turn's larger totals giving the differences; a new conversation
     restarting the baseline; none for zeros, a fall, model `''`, an unread version and a failed
     turn; `--version` run once); `usage_limit_reports_the_codex_window_and_reset` (decision 6: an
     answer in the page's shape with `primary` at 100, 300 minutes and `resetsAt` 1730948100
     giving 300 and `2024-11-07T02:55:00Z`; both at 100; another bucket at 100; a non-null
     `rateLimitReachedType` below 100; no `resetsAt`; nulls for none at 100, an error and a late
     answer; the blocked refresh; `limit: null` for other classes);
     `v4_maps_instructions_and_output_schema` (decisions 3 and 4: both flags on a first and a
     resumed launch, the file's bytes, `structured` or none, the refusal starting no client,
     `--disable memories` and `-c skills.include_instructions=false` on every version, `.v3`'s
     launch otherwise unchanged).
   - In `tests/package.rs`, `codex_v4_keeps_its_shape`: decisions 1 and 2.
   - In `agents.rs`, `codex_v4_registers_through_the_engine`, as 259's login test: a Pool's
     `codex-login.v1` Resource and two `codex.v4` Agents, one setting both places, one leaving
     them null, accepted as set.
   - Change an existing test only where decisions 1 to 7 require.
9. **The README.**
   - Before `## The standard Conduit templates`: `## The .v4 Agent` (decisions 1 and 3; why the
     schema is JSON text), `## Usage in each turn's result` (decision 5, each "no member" case),
     `## A usage limit's window and reset` (decision 6; DEC-046's pause of that account's lanes
     until `resetsAt`) and `## Model, effort and what stays off` (decision 4: a
     table of place, flag and default; what each flag keeps off; a skill the message names can
     still load). The failed turn's fields table gains `limit`.
   - Last, `## Moving to 0.2.4`: install, move the pin, replace each Agent; a `.v3` Agent
     replaced by a `.v4` one with the same `login` signs in nowhere.

## What to read

IFC-051, `usage.v1`, NAM-024, MAN-017, PRV-007; orders 259, 261; this package; `agents.rs`.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
grep -c '^  name: usage\.v1$' spec/core/results.yaml
d=packages/agent-codex; c=conformance/fixtures/agents/agent-codex
echo "$(grep -c '^version: 0\.2\.3$' $d/conduit.yaml) $(grep -c '^version = "0\.2\.3"$' $d/Cargo.toml) $(grep -cE '^- codex\.v[34]$' $d/conduit.yaml) $(grep -c '^version: 0\.2\.3$' packages/agent-claude-code/conduit.yaml)"
for f in conduit.yaml contracts.yaml; do cmp -s $d/$f $c/$f; echo "$f $?"; done
D=$(shasum -a 256 $c/contracts.tar.gz | cut -c1-64); git grep -l "sha256:$D" | wc -l | tr -d ' '
PYTHONDONTWRITEBYTECODE=1 python3 -c "import json; c=json.load(open('conformance/language/registration/agent-v2-packages/case.yaml')); print(*[len(s['expect']['reply']['package']['project']['documents']) for s in c['steps']])"
git grep -l -e 'conduit/usage' -e 'rateLimits/read' -- $d | wc -l | tr -d ' '
git grep -lE 'fn (codex_turns_report_their_usage|usage_limit_reports_the_codex_window_and_reset|v4_maps_instructions_and_output_schema|codex_v4_[a-z_]+)\(' | wc -l | tr -d ' '
test -e docs/work-orders/260-declared.yaml; echo "declared $?"
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
for p in engine $d; do (cd $p && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "$p $?"; done
(cd engine && cargo run --offline --release -p conduit-conformance --example make_archives -- --check 2>&1 | tail -n 1)
git status --short | wc -l | tr -d ' '
```

They print `1`, `1 1 1 1`, `conduit.yaml 0`, `contracts.yaml 0`, `7`, `9 9`, `0`, `0`,
`declared 1`, two lines ending ` 0`, `N reproducible archive fixtures verified` (keep N) and `0`.

## Scope

Change, in `packages/agent-codex`, `conduit.yaml`, `contracts.yaml`, `Cargo.toml`, `Cargo.lock`,
`README.md`, `src/` and `tests/`; the copy's `conduit.yaml`, `contracts.yaml`, `contracts.tar.gz`
and `README.md`; `live-agents/fixture.json`; the two cases' Codex install files, the binding
case's `conduit.json` and each `case.yaml`; `engine/examples/s1/conduit.yaml`; `agents.rs`;
`make_archives.rs`; and the new `docs/work-orders/260-declared.yaml`. Change nothing else: not
`spec/`, `packages/agent-claude-code` or its copy, another case or fixture, an engine source,
`experiments/` or a dependency; report what else needs it.

## Done when

Each line of the first group prints otherwise on `main` after orders 259 and 264 ("today") and
passes after:

```sh
ok='^test result: ok\. 1 passed'
d=packages/agent-codex; c=conformance/fixtures/agents/agent-codex
echo "$(grep -c '^version: 0\.2\.4$' $d/conduit.yaml) $(grep -c '^version = "0\.2\.4"$' $d/Cargo.toml) $(grep -c '^- codex\.v4$' $d/conduit.yaml) $(grep -c '^  name: codex\.v4$' $d/contracts.yaml) $(grep -c '^      structured: {type: object}$' $d/contracts.yaml) $(grep -cE '^## (The \.v4 Agent|Usage in each turn.s result|A usage limit.s window and reset|Model, effort and what stays off|Moving to 0\.2\.4)$' $d/README.md)"  # 1 1 1 1 1 5; today 0 0 0 0 0 0
for t in codex_turns_report_their_usage usage_limit_reports_the_codex_window_and_reset v4_maps_instructions_and_output_schema; do (cd $d && CARGO_NET_OFFLINE=true cargo test --offline --release --test wrapper $t 2>&1 | grep -c "$ok"); done  # 1 each; today 0
(cd $d && CARGO_NET_OFFLINE=true cargo test --offline --release --test package codex_v4_keeps_its_shape 2>&1 | grep -c "$ok")  # 1; today 0
(cd engine && CARGO_NET_OFFLINE=true cargo test --offline --release -p conduit-shell --test agents codex_v4_registers_through_the_engine 2>&1 | grep -c "$ok")  # 1; today 0
PYTHONDONTWRITEBYTECODE=1 python3 -c "import json; c=json.load(open('conformance/language/registration/agent-v2-packages/case.yaml')); print(*[len(s['expect']['reply']['package']['project']['documents']) for s in c['steps']])"  # 9 10; today 9 9
echo "$(grep -c 'at version 0\.2\.4 (work order 260)' $c/README.md) $(grep -c 'version: 0\.2\.4' engine/examples/s1/conduit.yaml)"  # 1 1; today 0 0
r=; for i in 1 2; do r="$r $(cd $d && CARGO_NET_OFFLINE=true cargo run --offline --locked --release --bin package 2>/dev/null | sed 's|^.*/dist/|dist/|')"; done; echo "$r"; tar -xOzf $d/dist/agent-codex.tar.gz conduit.yaml | grep -c '^version: 0\.2\.4$'  # one dist/agent-codex.tar.gz#SHA256 twice, then 1; today the 1 is 0
out=$(/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/260-declared.yaml); rc=$?; printf '%s\n' "$out" | tail -n 2; echo "exit $rc"  # 0 undeclared, 0 errors; exit 0; today exit 2
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
git status --short -uall | sed 's/^...//' | grep -vE '^(packages/agent-codex/(Cargo\.(toml|lock)|[a-z]+\.yaml|README\.md|src/.+|tests/.+)|conformance/fixtures/agents/agent-codex/[a-zA-Z.]+|conformance/language/registration/(live-agents|agent-v2-[a-z-]+)/[a-z-]+\.(json|yaml)|engine/examples/s1/conduit\.yaml|engine/crates/(shell/tests/agents|conformance/examples/make_archives)\.rs|docs/work-orders/260-declared\.yaml)$' | wc -l | tr -d ' '  # 0
(cd $d && CARGO_NET_OFFLINE=true cargo test --offline --release)  # every test passes
(cd engine && CARGO_NET_OFFLINE=true RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release)  # the whole engine suite, every vector among it
```

Report per decision what changed; each new test's failing, then passing line; both
`dist/agent-codex.tar.gz#SHA256` lines with `rustc -vV`'s `host:` and `release:`; each existing
test changed, and why; every check's last line. Stage nothing.

## What follows

- The role templates (DEC-047): worker and verifier on `codex.v4` with `model`, `effort`,
  `instructions` and `outputSchema`; and the pause until `resetsAt` (DEC-046), by order.
- The first real turns after the sitting check what no scripted client can: a resumed thread's
  running totals, both flags after `resume`, and the limit read's answer, each in a Record.

## Record

Run on 2026-11-22 through Conduit itself, by the dogfood's own Agents, in Pacific time, on
`main` with orders 258, 259 and 264 stacked while they waited in the merge queue.

**Worker-1's first turn** (03:39 to 03:40) stopped at the start check: the stack the coordinator
gave it predated orders 261 and 262, so `usage.v1` was absent. The coordinator rebuilt the stack
on `main`. **Its second turn** (03:42 to 03:58) released agent-codex 0.2.4: `codex.v4` with
optional `instructions` and `outputSchema`; each turn's usage as IFC-051's member, the difference
from the conversation's last running total; after a confirmed usage-limit stop, the window, its
length and the reset time read from the app-server with refresh blocked; memories and automatic
skill instructions off on every launch. 49 package tests and three vectors pass.

**Two blind reads at once** (03:59 to 04:03): `turn-verifier-1.turn.v1` found three musts; the
Claude reader passed it with two nits. One fix turn (04:04 to 04:14), each must reproduced first:
- a counter first reported on a resumed turn was attributed wholly to that turn; an optional
  counter whose earlier value is unknown is now omitted;
- the rate-limit read could hold the reply past its deadline during cleanup; one absolute
  deadline now bounds it;
- the role inputs' limits admitted values Linux cannot pass to a process; `instructions` now
  takes up to 21,841 characters and `outputSchema` up to 32,762, so each fits the 131,072-byte
  limit after escaping, with boundary tests; Codex's documentation available here named no file
  for additive instructions, so they stay inline;
- the README states the limits and that an empty string or a failing schema is refused; the
  reset time is formatted without `unsafe`.

53 package tests pass; the archive reproduces. The gate's run on the squash is the
coordinator's, at the merge.
