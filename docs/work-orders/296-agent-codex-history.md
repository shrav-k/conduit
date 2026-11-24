# Work order 296: agent-codex's `codex.v5` keeps each turn with its invocation and caller and offers `history.v1` (DEC-067, DEC-079)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; `vectors`
lists the three cases whose pins move. It needs no network: run cargo offline, and if a crate is
missing, stop and report. Run one cargo command at a time, and not `tools/gate.py` or the whole
engine suite. Never print a whole large file, case, diff or log: use `head`, `tail`, `grep`,
`sed -n` or a count. Never run `env`, `printenv`, `set` or `export -p`, never print an
environment variable's value, and never read or print a key, a token or any credential. Run no
`conduit` outside a test and no real `codex` or `claude`, sign in nowhere, and install nothing
into any store. Prefix each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is
public: no absolute local path, user, machine or account name, or private repository name.

## What and why

**The owner's answers.** [DEC-067](../decisions/log.md#dec-067-2026-11-22-an-agent-speaks-to-the-owner-only-in-replies-made-readable):
"Replies only, made readable (Recommended)": both Agent packages offer `history.v1`, each turn's
message, final text, invocation, caller and times, with no effect and no model work, to whoever
holds a Conduit listing it.
[DEC-079](../decisions/log.md#dec-079-2026-11-22-access-requests-health-and-history-now) builds it
now and keeps tool calls, tool results, file contents and usage limits out of it.

**Today** (on `main` at `a28e7fef`, 294 stacked). 0.2.5's newest Type, `codex.v4`, offers
`conversation.v1` through the plain item `wrapper-v4`. Neither the wrapper nor the engine keeps a
prompt or text (CTR-004). Order 294 lets a plain item list operations under `attributed`, whose
calls carry `io.github.shrav-k.conduit/invocation` and `io.github.shrav-k.conduit/caller`
(PKG-020, IFC-006, IFC-007). The engine sends them once DEC-067's engine order merges; these
tests send them themselves, so this order runs first.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 7,
everything an agent does is attributable: each turn is kept with its invocation and caller; 2,
every access can be inspected and revoked: `history` is its own template; 6, `history.v1` starts
no model work or client; 1, it holds no reasoning or tool call. None at risk (NAM-024).

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes,
by existing practice: a conversation service pages its messages by a cursor and a count, as
OpenAI's Conversations API (`after`, `limit` default 20, `order` asc) and Slack's
`conversations.history` (`oldest`, `limit`); oldest first is the coordinator's. Not taken:
Codex's session files, undocumented and holding tool calls; an operation in `conversation.v1`,
which every Type offering it would serve (TPL-004).

**When it runs.** Stacked on order 294 (`wo/294-plain-attributed`) until it merges; the start
check stops otherwise. Through Conduit: one Codex worker turn, the blind Codex verifier and
Claude reader at once, one fix turn, the gate (DEC-046, DEC-071, DEC-033). By ruling 89, each
decision says what must hold; how is yours. The coordinator's, open to the owner's strike
(DEC-029): the version 0.2.6, every name, bound and text, and the test names. No store installs
0.2.6 until a stage boundary or a dated amendment (DEC-080).

**Stop rule.** Stop, change nothing further, and report only for a contradiction between promises
(give both), a step that needs a change outside Scope (give its failing line), a file outside
Scope that must change, or a start-check line that prints otherwise. Where this order's wording
contradicts a promise it does not change, the promise governs: apply it and list it under
`own_choices`, beside every choice no decision governs.

## The decisions

1. **The contracts**, appended to `contracts.yaml` in this order; every earlier document keeps its
   bytes (NAM-024):
   - `codex.v5`: `codex.v4`'s document but its name; its description, adding what decision 3
     keeps and never keeps, in the state directory until the retire hook removes it;
     `offers: ['#conversation.v1', '#history-reading.v1']`; and `implementation`, `codex.v4`'s
     four entries and `'#history.v1'`, all to `'#wrapper-v5'`.
   - `history-reading.v1`, a Capability: `operations: ['#history.v1']`.
   - `history.v1`, an Operation: no `effect`, `startsModelWork: false`; optional inputs `since`,
     an integer from 0, default 0, and `count`, from 1 to 50, default 20; required outputs
     `turns`, at most 50 of decision 3's entries, and `latest`, an integer from 0. Each object
     states `required` and `additionalProperties: false`. Its `metadata.description` (IFC-033)
     gives decision 5's paging and decision 3's states, and says to read a `running` entry again
     from one below its index.
   - `history`, a Conduit: `capability: '#history-reading.v1'`, `operations: ['#history.v1']`
     (PKG-031).
2. **The manifest.** `version: 0.2.6`. `exports` gains `codex.v5` after `codex.v4`,
   `history-reading.v1` and `history.v1` after `new-conversation.v1`, and `history` last. A new
   item `wrapper-v5`, last: `wrapper-v4`'s lines, but `command: [bin/agent-codex, --shared-login,
   --history]`, `'#history.v1'` added to `implements` and to `tools` as `history`, and
   `attributed: ['#turn.v1']` after `plain: true`.
3. **What is kept.** Only with `--history`, one entry for each `turn` call that takes the
   session, a refusal for an earlier failed turn or a lapsed login among them, but none refused
   for its configuration or after the retire hook; so two attempts of one invocation give two:
   - `index`: 1, 2 and on, as turns start, never reused for the instance;
   - `invocation` and `caller`: the call's `_meta` values of those keys where strings, else null;
   - `message`: the turn's input; `text`: the result's `text` for a completed turn, else null;
   - `state`: `running` while this run serves the turn; `completed`; `failed` for any other
     reply; `interrupted` for an entry with no end that this run does not serve, as its
     wrapper stopped or could not write the end;
   - `startedAt` and `endedAt`: RFC 3339 UTC, such as `2026-11-22T10:11:12Z`; `endedAt` is null
     until the turn ends, and for `interrupted`.

   Nothing else: no tool call or result, file, reasoning, usage or limit (DEC-079), failure
   text, other client output, credential, environment or configuration value, or path.
4. **Where, and for how long.** `history.jsonl` in the state directory (PRV-029), beside
   `client-home`, mode 0600, written at each turn's start and at its end before the reply; a
   reader sees each entry whole or not at all. It lasts the instance's life, across new
   conversations and runs. With the state directory unset or relative, a turn answers its
   configuration refusal as today. A start that cannot be written fails the turn with
   `history (history.jsonl): cannot keep this turn in CONDUIT_STATE_DIR; no client started, the conversation is unchanged; call again once it is writable`;
   an end that cannot be written leaves the reply unchanged. With `--history`, after the stop
   and before emptying the home, the retire hook removes the file and answers
   `client stopped, its history removed and its home emptied; the login was left to its login Resource`,
   or fails with
   `client stopped; history (history.jsonl): cannot remove the history in CONDUIT_STATE_DIR; the home was not emptied`.
   Add no dependency.
5. **The tool `history`**, listed only with `--history`, its `inputSchema` the contract's
   (IFC-001), an absent input taking its default. Its `structuredContent` is `{turns, latest}`,
   and its text that JSON, as `new-conversation`'s: the entries whose `index` is above `since`,
   oldest first, at most `count`, and the highest index kept, 0 for none. It answers while a turn
   holds the session, without waiting, and starts no client; a missing file is an empty history.
   It refuses (ruling 52) with invalid params and
   `history: since must be a whole number from 0, the index of the last turn already read`,
   `history: count must be a whole number from 1 to 50` or
   `history: expected only since and count`; with the configuration's refusal where the retire
   hook gives it; and with the error results
   `history: session retired; its history is no longer served` after the retire hook and
   `history (history.jsonl): cannot read the history in CONDUIT_STATE_DIR`. Without `--history`
   the wrapper lists no `history` and answers it as an unknown tool.
6. **The README.** `## The .v5 Agent and its history`, after `## The .v4 Agent`, says plainly
   what is kept and never is, where, its mode, for how long and how it is removed (the retire
   hook; the state directory's removal, PRV-029, LIF-035, or the runtime provider's), that the
   engine keeps none, that `invocation` and `caller` read null until the engine sends them, the
   description's paging and states, and that the Agent's own tools can change the file (ruling
   70). The templates section adds `history`, which grants `history.v1` alone.
   Last, `## Moving to 0.2.6`: replace each Agent that should keep a history with a `codex.v5`
   one; the same `login` needs no sign-in. Fix each sentence the release makes false, such as
   the count of Agent Types.
7. **The release**, as [order 279](279-vendor-client-named-and-checked.md)'s decision 6 moved it,
   for Codex alone: `Cargo.toml` and the lockfile's own line; the Codex copy (both YAML files, the
   archive by `make_archives` in write mode, its README's `at version 0.2.6 (work order 296)`);
   Codex's pins in `live-agents/fixture.json`, `engine/examples/s1/conduit.yaml`, `agents.rs` and
   the two `registration-agent-v2-*` cases, the packages case's Codex step's `documents` gaining
   decision 1's four after `codex.v4`'s. `make_archives.rs`' 0.2.0 surface leaves those four out,
   so the install archives keep their bytes. Declare it in
   `docs/work-orders/296-declared.yaml`, in [279's](279-declared.yaml) form.

## The tests

Package tests only, on the scripted client through the wrapper's stdio with `--shared-login
--history` and a linked login, as the `.v4` tests run it, each call's `_meta` set by the test; no
engine test. Write them first, run them on the unchanged code and report each failing line. In
`tests/wrapper.rs`:

1. `v5_history_keeps_each_turn_with_its_invocation_and_caller`: `history` is listed with the
   contract's schema; a completed turn and a failed one, each with its own invocation and caller,
   a third with the second's invocation, refused as the session failed, and a fourth with
   neither key give four entries with decision 3's fields, `latest` 4, nulls in the fourth.
2. `v5_history_pages_by_since_and_count`: 21 turns; the defaults give 1 to 20; `since` 19 and
   `count` 1 give 20; `since` 21 gives none, `latest` 21; an empty history gives `latest` 0; each
   refusal of decision 5, for `since` -1 and `"1"`, `count` 0 and 51, and an extra input.
3. `v5_history_answers_while_a_turn_runs`: during a `BLOCK` turn, `history` shows it `running`
   with `endedAt` and `text` null; once released, `completed`.
4. `v5_history_holds_no_tool_call_file_or_usage`: before its final message the scripted client
   emits a command with output, an MCP tool call and result, a file change and a reasoning item,
   each with a canary, and usage; later turns, after `new-conversation`, fail with `FAIL:` and
   with a usage limit as the `.v4` usage-limit test scripts it. Neither `history` nor
   `history.jsonl` holds a canary, a count, a limit field, a failure text or the temporary path;
   the file's mode is 0600.
5. `v5_history_outlives_new_conversations_and_runs`: a turn, `new-conversation`, a turn, then a
   `BLOCK` turn whose wrapper is killed; a new wrapper on the same state directory gives 1 and 2
   `completed`, 3 `interrupted` with `endedAt` null, and its next turn index 4.
6. `v5_history_write_and_read_refusals`: with a directory at `history.jsonl`, a turn fails with
   decision 4's text, starting no client, and `history` gives its read refusal; once it is gone,
   the next turn completes in the same conversation.
7. `v5_retire_removes_the_history`: the hook answers as decision 4 says and removes the file;
   `history` then gives decision 5's retired refusal, and a refused turn writes no file. With a
   non-empty directory at `history.jsonl`, the hook fails with its text and the home keeps its
   entries.
8. `older_items_keep_no_history`: with `--shared-login` alone and with `.v2`'s environment, no
   `history` is listed, it is an unknown tool, and a turn writes no `history.jsonl`; with
   `--shared-login` alone, retire answers today's text.

In `tests/manifest.rs`, the item list gains `wrapper-v5`, and
`wrapper_v5_is_wrapper_v4_with_history` checks decision 2's lines. In `tests/package.rs`,
`codex_v5_keeps_its_shape` checks decision 1, and `standard_conduit_templates_keep_their_operations`
gains `history`'s copy. Change an existing test only where a decision requires it.

## What to read

The promises this order cites, as order 294 leaves them; the package whole; order 279;
`make_archives.rs`; `agents.rs`.

## Before you change anything

From the repository root:

```sh
export PYTHONDONTWRITEBYTECODE=1
x=packages/agent-codex; c=conformance/fixtures/agents/agent-codex; r=conformance/language/registration
git merge-base --is-ancestor a28e7fef HEAD; echo "main $?"
git status --short | wc -l | tr -d ' '
echo "$(grep -c '^    attributed:$' spec/schemas/manifest.yaml) $(grep -c 'It may also state `attributed`' spec/promises/packages.yaml)"
echo "$(grep -c '^version: 0\.2\.5$' $x/conduit.yaml) $(grep -c '^version = "0\.2\.5"$' $x/Cargo.toml) $(git grep -lE 'codex\.v5|history\.v1|history-reading|--history|history\.jsonl' -- $x | wc -l | tr -d ' ')"
for f in conduit.yaml contracts.yaml; do cmp -s $x/$f $c/$f; printf '%s ' $?; done; D=$(shasum -a 256 $c/contracts.tar.gz | cut -c1-64); git grep -l "sha256:$D" | wc -l | tr -d ' '
python3 -c "import json; c=json.load(open('$r/agent-v2-packages/case.yaml')); print(*[len(s['expect']['reply']['package']['project']['documents']) for s in c['steps']])"
sed -n '/^## DEC-067/,/^## DEC-068/p' docs/decisions/log.md | grep -c '^Status: accepted, not done'
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
for p in engine $x; do (cd $p && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "$p $?"; done
(cd engine && cargo run --offline --release -p conduit-conformance --example make_archives -- --check 2>&1 | tail -n 1)
```

They print `main 0`, `0`, `2 1` (`0 0` means 294 is not stacked: stop), `1 1 0`, `0 0 7`,
`11 10`, `1`, two lines ending ` 0` and `N reproducible archive fixtures verified` (keep N).

## Scope

Change, in `packages/agent-codex/`, both YAML files, `Cargo.toml`, `Cargo.lock` (its own line),
`README.md`, `src/` and `tests/`; the files decision 7 names, `agents.rs` in one version; and
create `docs/work-orders/296-declared.yaml`. Nothing else: not `spec/`, agent-claude-code or its
pins, an engine source, another case or fixture, the log, `experiments/` or a dependency.

## Done when

Each line of the first group prints otherwise on the stack today and passes after:

```sh
export PYTHONDONTWRITEBYTECODE=1
x=packages/agent-codex; c=conformance/fixtures/agents/agent-codex; r=conformance/language/registration
echo "$(grep -c '^version: 0\.2\.6$' $x/conduit.yaml) $(grep -c '^version = "0\.2\.6"$' $x/Cargo.toml) $(grep -cE '^- (codex\.v5|history-reading\.v1|history\.v1|history)$' $x/conduit.yaml) $(grep -c '^  wrapper-v5:$' $x/conduit.yaml) $(grep -c "^    attributed: \['#turn\.v1'\]$" $x/conduit.yaml)"  # 1 1 4 1 1; today 0 0 0 0 0
echo "$(grep -cE '^  name: (codex\.v5|history-reading\.v1|history\.v1|history)$' $x/contracts.yaml) $(grep -cE '^## (The \.v5 Agent and its history|Moving to 0\.2\.6)$' $x/README.md)"  # 4 2; today 0 0
(cd $x && cargo test --offline --release --test wrapper history 2>&1 | grep -c '^test result: ok\. 8 passed')  # 1; today 0
(cd $x && cargo test --offline --release --test manifest 2>&1 | grep -c '^test result: ok\. 2 passed')  # 1; today 0
(cd $x && cargo test --offline --release --test package codex_v5_keeps_its_shape 2>&1 | grep -c '^test result: ok\. 1 passed')  # 1; today 0
echo "$(grep -c 'at version 0\.2\.6 (work order 296)' $c/README.md) $(grep -c 'version: 0\.2\.6' engine/examples/s1/conduit.yaml) $(grep -c '("codex.v1", "0.2.6")' engine/crates/shell/tests/agents.rs)"  # 1 1 1; today 0 0 0
python3 -c "import json; c=json.load(open('$r/agent-v2-packages/case.yaml')); print(*[len(s['expect']['reply']['package']['project']['documents']) for s in c['steps']])"  # 11 14; today 11 10
s=; for i in 1 2; do s="$s $(cd $x && cargo run --offline --locked --release --bin package 2>/dev/null | sed 's|^.*/dist/|dist/|')"; done; echo "$s $(tar -xOzf $x/dist/agent-codex.tar.gz conduit.yaml | grep -c '^version: 0\.2\.6$')"  # one dist/agent-codex.tar.gz#SHA256 twice, then 1; today the 1 is 0
git add -N docs/work-orders/296-declared.yaml
out=$(/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/296-declared.yaml); rc=$?; printf '%s\n' "$out" | tail -n 2; echo "exit $rc"  # 0 undeclared, 0 errors; exit 0; today exit 2
```

Guards, in the same shell, which pass today and must still pass; the vectors test runs once,
last, and never the whole engine suite:

```sh
for f in conduit.yaml contracts.yaml; do cmp -s $x/$f $c/$f; printf '%s ' $?; tar -xOzf $c/contracts.tar.gz $f | cmp -s - $x/$f; printf '%s ' $?; done; D=$(shasum -a 256 $c/contracts.tar.gz | cut -c1-64); git grep -l "sha256:$D" | wc -l | tr -d ' '  # 0 0 0 0 7
(cd engine && cargo run --offline --release -p conduit-conformance --example make_archives -- --check 2>&1 | tail -n 1)  # the start check's line
git diff --quiet HEAD -- conformance/fixtures/install-archives packages/agent-claude-code conformance/fixtures/agents/agent-claude-code; echo "kept $?"  # kept 0
(cd $x && cargo fmt --check && cargo clippy --offline --all-targets -- -D warnings)
(cd engine && cargo fmt --all -- --check && cargo clippy --offline -p conduit-shell -p conduit-conformance --all-targets -- -D warnings)
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
for t in check_docs check_schemas check_private_paths; do python3 tools/$t.py | tail -1; done  # Errors: 0, three times
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(packages/agent-codex/(Cargo\.(toml|lock)|(conduit|contracts)\.yaml|README\.md|src/.+|tests/.+)|conformance/fixtures/agents/agent-codex/[a-zA-Z.]+|conformance/language/registration/(live-agents/fixture\.json|agent-v2-(packages|account-binding)/[a-z-]+\.(json|yaml))|engine/examples/s1/conduit\.yaml|engine/crates/(shell/tests/agents|conformance/examples/make_archives)\.rs|docs/work-orders/296-declared\.yaml)$' | wc -l | tr -d ' '  # 0
(cd $x && cargo test --offline --release 2>&1 | grep -cE '^(test result: FAILED|error)')  # 0: the package's suite
(cd engine && cargo test --offline --release -p conduit-shell --test agents 2>&1 | grep -c '^test result: ok\.')  # 1: the regressions
(cd engine && RUST_TEST_THREADS=4 cargo test --offline --release -p conduit-core --lib 2>&1 | grep -c '^test result: ok\.')  # 1: the core's tests
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release -p conduit-conformance --test vectors 2>&1 | grep -c '^test result: ok\.')  # 1: the vectors test, last
git reset -q -- docs/work-orders/296-declared.yaml  # stage nothing
```

Report per decision what changed; each new test's failing, then passing line; the `dist/` lines
with `rustc -vV`'s `host:` and `release:`; each existing test changed, and why; every check's last
line. Stage nothing.

## What follows

- agent-claude-code's order, in this shape; DEC-067's engine order, which sends the two keys;
  the role-template update. DEC-067's Status line is the coordinator's, after the last.

## Record

Run on 2026-11-22 through Conduit itself, by the dogfood's own Agents, in Pacific time, stacked
on spec order 294.

**Worker-2's turns** (16:44 to 17:10) built agent-codex 0.2.6: `codex.v5`, whose turn is
attributed, a wrapper that keeps each turn's message, final text, invocation, caller and times in
the instance's state directory, and `history.v1` with paging, no effect and no model work. A
first stop was the coordinator's: the stacked base's expected failures were out of ID order
after its rebase; the coordinator sorted them on 294's branch and the turn went on.

**Two blind reads** (17:11 to 17:15). The Codex verifier had four musts: text quoted in a final
answer, a refused turn still recorded, unchecked timestamps, and a scope line that came from the
working copy's older base. The Claude reader had one must (the README's grant used a key a
sectioned Agent does not take), two shoulds and a nit. The coordinator ruled on two, open to the
owner's strike: DEC-079's exclusions are of events, not of text, so the message and the Agent's
own final text are kept as written and the boundary is stated in the contract and README; the
scope line needs no change. One fix turn (17:16 to 17:31) applied all eight, each failing first:
75 package tests, the three registration vectors, 189 core tests and the vectors test pass. The
gate's run on the squash is the coordinator's, at the merge.
