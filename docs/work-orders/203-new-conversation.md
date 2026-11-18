# Work order 203: a new conversation without restarting the engine, in each Agent package (the full review's order 62)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access only for tests that
bind a loopback port; it adds no crate, so cargo fetches nothing, and you use the network for
nothing else. Run one cargo command at a time. Never print a whole large file: use `head`, `tail`,
`grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or `export -p`, never print an
environment variable's value, and never read or print a key, a token or any credential. Never
read or print any process's environment or arguments, never run a command that lists every
process on the machine, such as `ps -e`, `ps aux` or `top`, and never signal a process that a test
of this order did not start. Never run the real `codex` or `claude` programs, not even for their
help or version, and sign in nowhere. The tests use each package's scripted client, in temporary
directories only.

**Its number.** This order carries [the full review's](../proposals/full-review/orders.md) queue
row 62. It takes 203, the next number after 202 that no file uses, since 062 is the number of a
merged order of the condensation (the batch records' logs), whose Record is in
[the record](../implementation/WORK_ORDERS.md#order-062). When this order's Record is condensed,
its heading is `### Order 203`.

## What and why

**What it builds.** A caller that wants a fresh conversation with an Agent today has no operation
for it: `turn.v1` takes only `message`, a run holds one conversation, a failed turn ends it, and
nothing retires a slot, so the harness restarts the engine for each fresh conversation. This order
gives the capability `conversation.v1`, in each Agent package, a second operation,
`new-conversation.v1`, not marked as model work: it ends the Agent's conversation and starts a new
one, so that the next turn shares no context with the turns before, with no engine restart and no
new run. The packages are released as 0.2.1, and the engine's pinned copies of their contracts
move with them, so that the engine's test of the copies holds.

It carries [the full review's order 62](../proposals/full-review/orders.md):
[FR-339](../proposals/full-review/findings/P06-dogfooding.md#fr-339) as its verifier corrected it,
that is, "a separate operation in the capability ('start a new conversation', not marked as model
work)", so that "a Conduit could then grant turns without the right to discard an Agent's
context", and not an input on `turn.v1`, which would fail 050a's check of `.v1`. Its done-when:
"in each package, a test that the next turn shares no context, with no engine restart; 050a's
`.v1` check; Rust checks". Row 4-24 of
[increment 4's log](../proposals/first-build/log-increment-4.md) gives the same settlement and
adds that the operation "starts a new conversation and returns its ID". FR-339's other route,
continuing a named conversation, is not here: it works for Codex (`resume`), not for the Claude
Code wrapper, which runs its client with `--no-session-persistence`.

**When it is built: after 0.2.0 merges, as 0.2.1.** The coordinator decides that this order
starts from the tree in which order 050n has published the Agent packages 0.2.0, not on the 050
branch before 050n:

- The targeted review of the 0.2.0 surface froze `conversation.v1` with operations `['#turn.v1']`
  for 0.2.0 (verdict M04) and deferred "a slot for a further conversation operation (the full
  review's order 62)" to a later release (M06), in
  [its verdicts](../proposals/review-signin-2026-11-16/verdicts.md#a-the-release-and-the-contracts-it-carries)
  and [its finding V32](../proposals/review-signin-2026-11-16/findings.md#v32-m06-defers). Adding
  the operation on the 050 branch would change a member the review froze.
- NAM-024, as [order 199](199-nam-024-which-additions-count.md) worded it, makes the addition one
  a later release at the same address may make: "A further operation of a capability is one of
  them, and so is the default implementation of it that a Type whose earlier contract offers that
  capability adds to its `implementation`". So nothing is lost by waiting for the freeze.
- The full review's row puts it after "050 published; #17", with its place "After" the pin, and
  the log's row 4-24 says "once order 050 has published 0.2.0". On the 050 branch it would join
  0.2.0's serial chain, which waits on 048b-join and a live check, and lengthen it.

The cost: the harness keeps its restart until 0.2.1 is installed.

**Where this order starts.** Your tree is the one the coordinator names as `BASE`: `main` after
order 050n has merged, or, by the coordinator's dispatch note under stacked dispatch, the tip of
050n's verified branch. Under stacked dispatch, the coordinator rebases 050n's verified branch onto
`main` once this order's text has merged, and sets `BASE` to its tip. Either way `BASE` holds
order 050n's work and, where it is on `main`, its Record, and its tip holds this order file,
committed. Order 050n brings:
both packages at version 0.2.0 in `conduit.yaml` and `Cargo.toml`; `conversation.v1` with
operations `['#turn.v1']`, and each item that implements `turn.v1` with an `implements` line that
starts with `'#turn.v1'`: exactly `['#turn.v1']` in the `.v1` item, and order 050h's
`['#turn.v1', 'github.com/shrav-k/conduit/core#retire-hook.v1']` in each `.v2` item; order 050h's
`retire` tool, which each `.v2` item lists after `turn`, and its README sentence; 050a's check passing (the 0.1.1 contracts a byte prefix of each
`contracts.yaml`); the wrapper sources as orders 050a to 050m leave them, the two `lib.rs` one
source but for the `CLAUDE` constant and the two `main.rs` one file; and the engine's copies at
0.2.0: each package's `conduit.yaml` and `contracts.yaml` equal to its copy under
`conformance/fixtures/agents/`, each copy's `contracts.tar.gz` holding those two copies, its
digest pinned in `conformance/language/registration/live-agents/fixture.json` and
`engine/examples/s1/conduit.yaml`, and `engine/crates/shell/tests/agents.rs` pinning version
0.2.0. Orders 050f, 050g and 050h are written, and the checks and decisions below follow their
texts; orders 050i to 050m were not written when this order was, so the coordinator re-reads
050n's tree before dispatch and revises the checks below and decisions 4 and 5 where that tree
differs from what they say. Before you change anything, check from the repository root that you
find there what that gives, and stop, change nothing and report if any check prints otherwise:

```bash
git merge-base --is-ancestor e4ceb4c8 "$BASE"; echo "main $?"
git merge-base --is-ancestor "$BASE" origin/main; echo "merged $?"
test "$(git rev-parse HEAD)" = "$(git rev-parse "$BASE")"; echo "head $?"
git ls-files --error-unmatch docs/work-orders/203-new-conversation.md >/dev/null; echo "order $?"
test -e docs/work-orders/203-declared.yaml; echo "declared $?"
git status --short | wc -l
ls docs/work-orders/050n-*.md | wc -l
grep -c '^## Record' docs/work-orders/050n-*.md
grep -c '^version: 0.2.0$' packages/agent-codex/conduit.yaml packages/agent-claude-code/conduit.yaml
grep -c '^version = "0.2.0"$' packages/agent-codex/Cargo.toml packages/agent-claude-code/Cargo.toml
grep -c "^  operations: \['#turn.v1'\]$" packages/agent-codex/contracts.yaml packages/agent-claude-code/contracts.yaml
for NAME in agent-codex agent-claude-code; do N=$(( $(git show e4ceb4c8:packages/$NAME/contracts.yaml | wc -c) )); head -c "$N" packages/$NAME/contracts.yaml | cmp -s - <(git show e4ceb4c8:packages/$NAME/contracts.yaml); echo "$NAME 0.1.1 $?"; done
grep -rlE 'new-conversation|new_conversation' packages/agent-codex packages/agent-claude-code --exclude-dir=target --exclude-dir=dist | wc -l
grep -c 'pub async fn turn(&mut self, message: &str) -> Result<Value>' packages/agent-codex/src/lib.rs packages/agent-claude-code/src/lib.rs
grep -c 'pub async fn shutdown(&mut self) -> StopReport' packages/agent-codex/src/lib.rs packages/agent-claude-code/src/lib.rs
grep -cE '^    (thread: Option<String>|failed: Option<\(usize, String\)>|turns_started: usize),$' packages/agent-codex/src/lib.rs packages/agent-claude-code/src/lib.rs
grep -c 'self.thread = None' packages/agent-codex/src/lib.rs packages/agent-claude-code/src/lib.rs
grep -c 'if r.name != "turn"' packages/agent-codex/src/main.rs packages/agent-claude-code/src/main.rs
grep -c '"name":"retire"' packages/agent-codex/src/main.rs packages/agent-claude-code/src/main.rs
diff packages/agent-codex/src/lib.rs packages/agent-claude-code/src/lib.rs | grep -c '^[<>]'
diff packages/agent-codex/src/main.rs packages/agent-claude-code/src/main.rs | grep -c '^[<>]'
for NAME in agent-codex agent-claude-code; do tr '\n' ' ' < packages/$NAME/README.md | grep -c 'The plain MCP wrapper exposes `turn`, and each `.v2` item also `retire`; the manifest maps each versioned operation or hook to its tool\.'; done
grep -c '^K counts turns from 1 and continues across resumes\.' packages/agent-codex/README.md packages/agent-claude-code/README.md
grep -c '^## The .v2 failed turn.s fields$' packages/agent-codex/README.md packages/agent-claude-code/README.md
/usr/bin/python3 -c 'import sys, yaml
for p in sys.argv[1:]:
    d = [x for x in yaml.safe_load_all(open(p)) if x]
    print(p.split("/")[1], sum(1 for x in d if x["kind"] == "Type" and "#turn.v1" in x["spec"].get("implementation", {})), [x["metadata"]["name"] for x in d if x["kind"] == "Capability"])' packages/agent-codex/contracts.yaml packages/agent-claude-code/contracts.yaml
for NAME in agent-codex agent-claude-code; do test "$(grep -cE "^    implements: \['#turn\.v1'(, 'github\.com/shrav-k/conduit/core#retire-hook\.v1')?\]$" packages/$NAME/conduit.yaml)" -eq "$(grep -c "implements:.*'#turn\.v1'" packages/$NAME/conduit.yaml)"; echo "$NAME implements $?"; done
grep -c "^    implements: \['#turn\.v1', 'github\.com/shrav-k/conduit/core#retire-hook\.v1'\]$" packages/agent-codex/conduit.yaml packages/agent-claude-code/conduit.yaml
for NAME in agent-codex agent-claude-code; do for F in conduit.yaml contracts.yaml; do cmp -s packages/$NAME/$F conformance/fixtures/agents/$NAME/$F; echo "$NAME $F $?"; tar -xOzf conformance/fixtures/agents/$NAME/contracts.tar.gz $F | cmp -s - conformance/fixtures/agents/$NAME/$F; echo "$NAME archive $F $?"; done; done
for NAME in agent-codex agent-claude-code; do D=$(shasum -a 256 conformance/fixtures/agents/$NAME/contracts.tar.gz | cut -c1-64); grep -c "sha256:$D" conformance/language/registration/live-agents/fixture.json engine/examples/s1/conduit.yaml; done
grep -c 'version: 0\.2\.0\\n    digest' engine/crates/shell/tests/agents.rs
grep -c 'releases.push(("agents-0.2.0".into(), manifest, docs));' engine/crates/conformance/examples/make_archives.rs
for NAME in agent-codex agent-claude-code; do tar -tvzf conformance/fixtures/agents/$NAME/contracts.tar.gz | awk '{$5 = "SIZE"; print}'; done
(cd engine && cargo run --release -p conduit-conformance --example make_archives -- --check 2>&1 | tail -n 1)
```

They print `main 0`; `merged 0` (one of the two lines the coordinator's dispatch note may relax,
where `BASE` is 050n's verified branch and not yet on `main`); `head 0`, `order 0`, `declared 1`
(no declaration yet: you write it, decision 7) and `0`; `1` and `1` (050n's order file and its
Record, the other line the dispatch note may relax, since a merging order's Record is written at
its merge and 050n's verified branch has none); `1` for each of the two `conduit.yaml`, the two
`Cargo.toml` and the two `contracts.yaml`; `agent-codex 0.1.1 0` and `agent-claude-code 0.1.1 0` (050a's
check, against the 0.1.1 contracts as `main` held them at `e4ceb4c8`); `0` (no file holds the new
names yet); `1` for each `lib.rs` (`turn`), `1` for each (`shutdown`), `3` for each (the three
fields of `Session`), `0` for each (`self.thread = None`), `1` for each `main.rs` (the tool check)
and `1` for each (order 050h's `retire` tool); `2` (the `CLAUDE` line); `0`; `1` and `1` (the
README's sentence as order 050h leaves it); `1` for each README (the turn count's sentence) and
`1` for each (050e's section); and
`agent-codex 2 ['conversation.v1']` and `agent-claude-code 2 ['conversation.v1']` (two Types in
each package implement `turn.v1`, and the one capability is `conversation.v1`);
`agent-codex implements 0` and `agent-claude-code implements 0` (each item's `implements` line
is `['#turn.v1']`, or that with 050h's `retire-hook.v1` after it); `2` for Codex's `conduit.yaml`
and `1` for Claude Code's (the `.v2` items, `wrapper-v2` and, in `agent-codex`,
`wrapper-v2-path`, each with `retire-hook.v1` after `'#turn.v1'`); for each package and each of `conduit.yaml` and `contracts.yaml`,
`NAME FILE 0` and `NAME archive FILE 0` (the copy equals the package's file, and the archive holds
the copy); for each package, `1` for `fixture.json` and `1` for `conduit.yaml` (the archive's
digest is pinned in both); `1` (the test's version pin); `1` (the generator's release
`agents-0.2.0`, which decision 7 changes); for each package, two lines, `conduit.yaml` then
`contracts.yaml`, each of mode `-rw-r--r--`, owner and group `0` and time 0 (the epoch, shown in
local time), its size masked (keep them: the done-when compares the rebuilt archives' listings
with them); and
`N reproducible archive fixtures verified` (keep its line). `/usr/bin/python3`
is the Mac's interpreter with PyYAML; where `python3` has PyYAML, it serves as well. Then record,
for the report, the number of `#[test]` and `#[tokio::test]` attributes in each
`tests/wrapper.rs` at `BASE`: the done-when counts one more; the number of tools that each
`list_tools` lists at `BASE`, for `.v1` and for `.v2` (at this writing, `turn` for `.v1`, and
`turn` then `retire` for `.v2`); and the line count that the `lib.rs`
diff above printed (`2` at this writing).

**What the tree already does, so that you do not redo it.** In each package's `src/lib.rs` and
`src/main.rs`, as 050a to 050e built them and later 050 orders may have extended them:

- `Session` keeps `stream` (the running client, where one runs), `thread` (Codex's thread ID,
  which later turns resume; Claude Code never sets it), `failed` (the turn number and cause of a
  failed turn, after which every turn is refused with
  `session failed at turn K: CAUSE; implicit restart refused`) and `turns_started` (the turns of
  this conversation; a failed turn's `FailedTurn` takes its number from it, and the client tool
  report is `reported` and `current` only where it is 1).
- `Session::shutdown` drops the client's stdin, stops the client's process tree for `.v2` (order
  050d's `ClientTree::stop(false)`, which does not close the tree, so a later launch is allowed),
  kills and reaps the client, sets `stream` to `None` and returns 050e's `StopReport`.
- `Session::turn` launches a client where `stream` is `None`, with `arguments(config, thread)`:
  for Codex, `resume THREAD` where `thread` is set; for Claude Code, one process for the whole
  conversation, run with `--no-session-persistence`.
- `main.rs`'s `list_tools` lists `turn`, and for a `.v2` wrapper `retire` after it (order 050h),
  and `call_tool` serves `retire` for `.v2` and refuses any other name as invalid parameters
  (`unknown tool`). `Session` holds 050h's `retired`, set once the retire hook begins, after
  which every turn is refused.
- The scripted client answers a message such as `TEST` with a JSON text of its evidence, holding
  among others `args`, `pid` and `number`: for Codex, `2` where it was resumed and `1` otherwise;
  for Claude Code, the line's index in that client process, from `1`. `FAIL:TEXT` fails the turn.

## The decisions it applies

**1. The contract.** In each package's `contracts.yaml`, make exactly these three changes and no
other:

- In the Capability `conversation.v1`, the line `  operations: ['#turn.v1']` becomes
  `  operations: ['#turn.v1', '#new-conversation.v1']`.
- In each Type's `spec.implementation` (the `.v1` Type and the `.v2` Type, so two in each
  package), add the key `'#new-conversation.v1'` with the same value as that map's
  `'#turn.v1'` key, right after the `'#turn.v1'` entry and in the map's own style: in a block
  map, a line with the `'#turn.v1'` line's indent,
  `'#new-conversation.v1': '#ITEM'`; in a flow map, `, '#new-conversation.v1': '#ITEM'` right
  after `'#turn.v1': '#ITEM'`. TPL-004 needs this default in every Type that offers
  `conversation.v1`, the `.v1` Types included (V32).
- Append, at the end of the file, after its last line, this document, byte for byte:

```yaml
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Operation
metadata:
  name: new-conversation.v1
spec:
  effect: >-
    Ends the Agent's conversation and stops its client, as a failed turn does, and starts a new
    one: the next turn.v1 shares no context with the turns before it. The Agent's run goes on.
  startsModelWork: false
  destructiveHint: true
  inputSchema:
    type: object
    properties: {}
    additionalProperties: false
  outputSchema:
    type: object
    properties:
      conversation: {type: integer, minimum: 2}
      ended:
        type: object
        properties:
          turns: {type: integer, minimum: 0}
          failed: {type: boolean}
          survivors:
            type: array
            items: {type: integer}
          stopConfirmed: {type: boolean}
        required: [turns, failed, survivors, stopConfirmed]
        additionalProperties: false
    required: [conversation, ended]
    additionalProperties: false
```

No description changes, `turn.v1` does not change, and no Type's `offers`, `schema`, `requires`,
`signIn` or `records` changes: NAM-024 allows only the additions above. So removing exactly them
gives back 0.2.0's file byte for byte, and 0.1.1's file stays its byte prefix: that is 050a's
`.v1` check as NAM-024 bounds it for a later release. 050a's command run on the 0.2.1 file itself
fails, at the `conversation.v1` line, by design: V32 records that the `.v1` contracts change in
the release that adds the operation, "which 050a decision 10's byte-for-byte rule (a 0.2.0
decision) does not govern".

**2. The manifest and the version.** In each package's `conduit.yaml`: `version: 0.2.0` becomes
`version: 0.2.1`; in `exports`, add the line `- new-conversation.v1` right after `- turn.v1`; and
in each item whose `implements` list holds `'#turn.v1'`, append `'#new-conversation.v1'` to that
list right after `'#turn.v1'`, and add to its `tools` map, right after the `'#turn.v1'` line and
with its indent, `'#new-conversation.v1': new_conversation`. No other item and no other member
changes, so `agent-codex`'s `wrapper-v2-path` still differs from `wrapper-v2` only in its program
(the manifest test). In each `Cargo.toml`, `version = "0.2.0"` becomes `version = "0.2.1"`, and
`Cargo.lock` changes only in the package's own `version` line, as cargo rewrites it.

**3. The session.** In each `src/lib.rs`, add to `Session`, after `turns_started: usize,`, the
field `conversation: usize,`, set to `1` in `Session::new`. Add, right after `Session::cancel`:

```rust
    /// Ends this conversation and starts the next (the full review's order 62). The client stops
    /// as at a failed turn; the thread, the failure and the turn count are forgotten, so the next
    /// turn launches a fresh client that resumes nothing. It launches no client itself.
    pub async fn new_conversation(&mut self) -> Result<Value> {
        if let Some(error) = &self.config.configuration_error {
            bail!("{error}");
        }
        let turns = self.turns_started;
        let failed = self.failed.is_some();
        let stop = self.shutdown().await;
        self.thread = None;
        self.failed = None;
        self.turns_started = 0;
        self.conversation += 1;
        Ok(json!({"conversation":self.conversation,"ended":{"turns":turns,"failed":failed,"survivors":stop.survivors,"stopConfirmed":!stop.unconfirmed}}))
    }
```

`Session::turn`, `run_turn`, `cancel`, `end_failed_turn` and `shutdown` do not change. So a turn's
number, in its failure text and in 050e's fields, counts from 1 in each conversation, and the
first turn of a new conversation reports its client tools as `reported` and `current`. A failed
conversation stays ended: its later turns are refused as before, until a `new-conversation.v1`
call starts the next one, which is an explicit request, not an implicit restart. `new_conversation`
does not touch order 050h's `retired`: once the retire hook has begun, every turn is still refused
with `RETIRED_REFUSAL`, whatever conversation follows.

**4. The tool.** In each `src/main.rs`:

- In `list_tools`, after the line that builds the `turn` tool (`let tool = ...;` at this
  writing), add

  ```rust
  let new_conversation = serde_json::from_value(json!({"name":"new_conversation","description":"Explicitly end this Agent's conversation and start a new one; the next turn shares no context with the turns before. Starts no model work.","inputSchema":{"type":"object","properties":{},"additionalProperties":false}})).unwrap();
  ```

  and append `new_conversation` to the list after the tools that `BASE` lists, so that the list
  holds one more tool than at `BASE` and the tools before keep their order (at this writing,
  `turn` then `new_conversation` for `.v1`, and `turn`, `retire`, then `new_conversation` for
  `.v2`, so `turn` stays first and `new_conversation` last).
- In `call_tool`, before the check `if r.name != "turn"`, add

  ```rust
  if r.name == "new_conversation" {
      if r.arguments.as_ref().is_some_and(|args| !args.is_empty()) {
          return Err(ErrorData::invalid_params("expected no arguments", None));
      }
      let cancelled = || ErrorData::internal_error("new conversation cancelled", None);
      let mut session = tokio::select! {
          biased;
          _ = context.ct.cancelled() => return Err(cancelled()),
          session = self.0.lock() => session,
      };
      if context.ct.is_cancelled() {
          return Err(cancelled());
      }
      let result = match session.new_conversation().await {
          Ok(value) => {
              let mut result = serde_json::from_value::<CallToolResult>(
                  json!({"content":[{"type":"text","text":value.to_string()}],"structuredContent":value,"isError":false}),
              )
              .unwrap();
              result.result_type = Some(ResultType::COMPLETE);
              result
          }
          Err(error) => CallToolResult::error(vec![ContentBlock::text(error.to_string())]),
      };
      return Ok(result.into());
  }
  ```

  A call queued behind a running turn waits for the session's lock, as a second turn does, and a
  cancellation before it holds the lock changes nothing. Once it holds the lock it runs to its
  end, which 050d's bounded stop keeps short. Where the wrapper's TERM stop has begun, the call
  waits for the wrapper's exit, as 050d's `stop(false)` does. Format with `cargo fmt`; the two
  `main.rs` stay one file.

**5. The tests**, in each package's `tests/wrapper.rs`, the same in both:

- **`stdio_exposes_only_turn_and_setup_never_launches_client`** is renamed
  `stdio_exposes_turn_and_new_conversation_and_setup_never_launches_client`. It asserts the tools
  that `BASE` lists, in their order, then `new_conversation` (at this writing two tools, `turn`
  then `new_conversation`), and that the second's input schema, as JSON, equals
  `{"type":"object","properties":{},"additionalProperties":false}`. Right after its check that
  `client-started` does not exist, it calls `new_conversation` with `{"reset":true}`, which must be
  an error, then with `{}`, whose result has `isError` false and `structuredContent`
  `{"conversation":2,"ended":{"turns":0,"failed":false,"survivors":[],"stopConfirmed":true}}`,
  and checks again that `client-started` does not exist. The rest of the test does not change.
- **`stdio_v2_defaults_and_invalid_configuration_remain_callable`** asserts the tools that `BASE`
  lists, then `new_conversation`, one more than it asserted (at this writing, as order 050h
  leaves it, `["turn", "new_conversation"]` for the case `v1` and
  `["turn", "retire", "new_conversation"]` for every other case). In each case with an invalid configuration, after
  its turns, a `new_conversation` call with `{}` answers `isError` true with one text block, equal
  to the turns' `invalid configuration: ...` text.
- **A new test, `new_conversation_shares_no_context_without_restart`**, through the wrapper binary
  over stdio, for `v1` and then `v2`, each with a fresh temporary directory and wrapper, started
  as `v2_failed_turn_fields_reach_the_result` starts it (with `AGENT_ACCOUNT_SERVICES` `false`;
  for `v2`, `AGENT_TURN_DEADLINE_SECONDS` `60` and `CONDUIT_STATE_DIR` the config's; for `v1`,
  `AGENT_PEER_MESSAGING` `false`), with a plain client (`().serve(...)`). It reads the wrapper's
  PID once, and after every call asserts that `child.id()` is still that PID and
  `child.try_wait()` gives `None`: one wrapper, the Agent's run, serves the whole case, and nothing
  restarts it. It reads each turn's evidence by parsing `structuredContent.text`. In order:
  1. `turn` `TEST` succeeds; its evidence's `number` is `1`; for Codex its `args` hold no
     `resume`; `clientTools.status` is `reported`.
  2. `turn` `TEST` succeeds with `number` `2`; for Codex its `args` hold `resume` and
     `fixture-exact-thread`; for Claude Code its `pid` equals turn 1's; `clientTools.status` is
     `unavailable`. The two turns shared one conversation.
  3. `new_conversation` `{}` succeeds: `isError` false, `structuredContent`
     `{"conversation":2,"ended":{"turns":2,"failed":false,"survivors":[],"stopConfirmed":true}}`,
     and one text block that parses as JSON to the same value.
  4. `turn` `TEST` succeeds. First, for Codex, its `args` hold neither `resume` nor
     `fixture-exact-thread`; for Claude Code, its `pid` differs from turn 2's. Then its `number`
     is `1`, and `clientTools.status` is `reported` with the first observation's `freshness`
     `current`. This turn shares no context with turns 1 and 2.
  5. `turn` `FAIL:stream disconnected before completion` answers `isError` true, its first block
     `client turn 2 failed: client failure: network; session cannot restart implicitly`.
  6. `turn` `TEST` answers `isError` true with one block,
     `session failed at turn 2: client failure: network; implicit restart refused`.
  7. `new_conversation` `{}` succeeds with `structuredContent`
     `{"conversation":3,"ended":{"turns":2,"failed":true,"survivors":[],"stopConfirmed":true}}`.
  8. `turn` `TEST` succeeds with `number` `1`, and for Codex `args` that hold no `resume`.
  9. `new_conversation` with `{"reset":true}` is an error, and a following `turn` `TEST`
     succeeds with `number` `2`: the refused call changed nothing.

  The test `eprintln!`s each `new_conversation` result and each turn's `number` and, for Claude
  Code, `pid`. At its end it cancels the service, and the wrapper's stderr holds none of
  `sentinel-secret`, `synthetic-test-token`, `synthetic-anthropic-key` and
  `synthetic-openai-key`.

- **`v2_retire_tool_through_the_wrapper`**, order 050h's, changes in its two tool lists alone:
  the `.v2` wrapper's listed tools' names are `["turn", "retire", "new_conversation"]`, and the
  `.v1` wrapper's `["turn", "new_conversation"]`. Any other test at `BASE` that asserts the listed
  tools gains `new_conversation` at the end in the same way, and nothing else in it changes;
  report each such test.

The test count goes up by one in each package. Every other test keeps passing unchanged.

**6. The READMEs**, both, in the package's own client's terms. Write no absolute local path and no
client version.

- The sentences "Its manifest exports two Agent Types, a conversation capability and the explicit
  `turn.v1` operation. The plain MCP wrapper exposes `turn`, and each `.v2` item also `retire`; the
  manifest maps each versioned operation or hook to its tool." become "Its manifest exports two
  Agent Types, a conversation capability and its two operations, the explicit `turn.v1` and
  `new-conversation.v1`. The plain MCP wrapper exposes `turn` and `new_conversation`, and each
  `.v2` item also `retire`; the manifest maps each versioned operation or hook to its tool.",
  wrapped as the paragraph is.
- "K counts turns from 1 and continues across resumes." becomes "K counts turns from 1 and
  continues across resumes; a new conversation counts from 1 again.", with "new conversation" a
  link to the new section's anchor, `a-new-conversation-both-versions`.
- Add, at the end, `## A new conversation (both versions)`, which says: from 0.2.1 the capability
  `conversation.v1` holds a second operation, `new-conversation.v1`, which the wrapper exposes as
  the tool `new_conversation`; it takes no input and starts no model work, so a Conduit can grant
  turns without it, or it without turns; a call ends the Agent's conversation, stopping the client
  as at a failed turn (for `.v2`, its whole process tree) and forgetting the conversation's
  thread, its failure and its turn count, and launches no client; the next `turn` then shares no
  context with the turns before: for `agent-codex`, "it launches Codex with no `resume`, so Codex
  starts a new thread"; for `agent-claude-code`, "it launches a new Claude Code process, and since
  the wrapper runs Claude Code with `--no-session-persistence`, nothing of the ended conversation
  is kept to resume"; the Agent's run and the engine go on, so a fresh conversation needs no
  restart; after a failed turn the failed conversation stays ended and its later turns are
  refused, and `new_conversation` starts the next. Then the result: its `structuredContent`, also
  given as JSON text, is `{"conversation":N,"ended":{"turns":T,"failed":F,"survivors":[...],"stopConfirmed":B}}`,
  where N numbers the conversation now started, counting the run's first as 1, within one run of
  the wrapper only (a new run counts from 1 again), and `ended` gives the turns the ended
  conversation started, whether one failed, and, as in the failed turn's fields (a link to
  `the-v2-failed-turns-fields`), the PIDs that outlived the stop and whether
  the stop was confirmed (for `.v1`, always none and true); an input is refused as invalid
  parameters, an invalid configuration answers as a turn does, and a call waits for a running
  turn. And: the 0.2.1 contracts are 0.2.0's with NAM-024's additions alone, the operation in
  `conversation.v1`'s list, its Operation contract and each Type's default implementation of it,
  the `.v1` Types included.

**7. The engine's copies.** The engine's test
`engine/crates/shell/tests/agents.rs`, which `tools/gate.py` runs for any change under the two
packages, requires each package's `conduit.yaml` and `contracts.yaml` to equal its copy under
`conformance/fixtures/agents/` byte for byte, and registers an archive built from the package
under a pinned version (PKG-007 refuses a pin whose version or digest does not match). The
generator of the install-archive fixtures, `make_archives`, builds its release `agents-0.2.0` from
one of those copies, and its `--check` compares every byte it would write. So the copies move to
0.2.1 with the packages, and the generator keeps 0.2.0's release as it is, in exactly these files,
with the declaration of the vector changes they make:

- In each of `conformance/fixtures/agents/agent-codex/` and
  `conformance/fixtures/agents/agent-claude-code/`: `conduit.yaml` and `contracts.yaml` become
  byte-for-byte copies of the package's new files; `contracts.tar.gz` is rebuilt from those two
  copies alone and holds nothing else; and in `README.md`, the version it names, 0.2.0, becomes
  0.2.1, and the order named beside it, where `BASE` names one, becomes
  `(work order 203, the full review's order 62)`; nothing else changes, and the line keeps its
  wrapping. Any `contracts.tar.gz` is
  acceptable whose two members equal the copies and whose `tar -tvzf` listing, with sizes masked,
  equals `BASE`'s (the same member names in the same order, modes 0644, owner and group 0, time
  0), since its digest is pinned from the file built. Where 050n's Record names the command that
  built it for 0.2.0, use that command; otherwise choose one, and report it as your own choice.
- In `engine/crates/conformance/examples/make_archives.rs`, where the release `agents-0.2.0` takes
  the non-Type documents of `conformance/fixtures/agents/agent-claude-code/contracts.yaml`, leave
  out the Operation `new-conversation.v1` too, and remove `'#new-conversation.v1'` from
  `conversation.v1`'s `operations`, so that the release keeps 0.2.0's shape. A fixture named
  0.2.0 does not hold 0.2.1's operation, and so
  `conformance/fixtures/s1/packages/agents-0.2.0`, its archive under
  `conformance/fixtures/install-archives/`, `digests.json` and the `agents-v2-defaults` vector's
  pinned digest stay byte for byte: do not regenerate them. Nothing else in the file changes.
- In `conformance/language/registration/live-agents/fixture.json` and
  `engine/examples/s1/conduit.yaml`: for each Agent package, its `version` becomes `0.2.1` and its
  `digest` the `sha256:` digest of the copy's new `contracts.tar.gz`. Nothing else changes.
- In `engine/crates/shell/tests/agents.rs`: the pin `version: 0.2.0` becomes `version: 0.2.1`.
  Nothing else changes.
- **The declaration.** The case `registration-live-agents` names eight of these files as fixture
  aliases: `fixture.json`, `engine/examples/s1/conduit.yaml` and, for each package, the copy's
  `conduit.yaml`, `contracts.yaml` and `contracts.tar.gz`. `tools/vector_delta.py` compares each
  alias as bytes and reports each changed one as a `fixture` change, and the local gate's row
  `vector changes` fails on any change that no declaration covers. So write the new file
  `docs/work-orders/203-declared.yaml`, in this build and not before: the gate reads a
  declaration through `--declared-changed` only where the branch adds or changes it from its merge
  base, and this order's text is already on `main`. It holds this one shape, byte for byte:

```yaml
- name: the copies and pins of the Agent packages move to 0.2.1
  kind: fixture
  cases: [registration-live-agents]
  paths:
  - conformance/language/registration/live-agents/fixture.json
  - engine/examples/s1/conduit.yaml
  - conformance/fixtures/agents/agent-codex/conduit.yaml
  - conformance/fixtures/agents/agent-codex/contracts.yaml
  - conformance/fixtures/agents/agent-codex/contracts.tar.gz
  - conformance/fixtures/agents/agent-claude-code/conduit.yaml
  - conformance/fixtures/agents/agent-claude-code/contracts.yaml
  - conformance/fixtures/agents/agent-claude-code/contracts.tar.gz
```

  Change it only to match the change: if `vector_delta.py` reports a fixture change of
  `registration-live-agents` outside these eight paths, or a change of any other case, or any
  `case.yaml` change, stop and report it; do not widen the shape.

No vector's `case.yaml`, no promise and no other test changes; if the engine's tests need one to
change, stop and report. Then run `python3 tools/promise_standing.py --check`; if the engine's S1
count moved and it reports `engine/STANDING.md` stale, run it with `--write`, which may change
that file alone, and report the change.

**8. What this order does not do.** No change to `turn.v1`, to any description, or to any
`schema`, `offers`, `requires`, `signIn` or `records`; no input on `turn.v1`; no continuing of a
named conversation; no conversation ID in a turn's output; no change to `tests/fixtures/client.rs`,
`src/bin/package.rs`, `tests/package.rs` or `tests/manifest.rs`; and none under `engine/`,
`conformance/`, `spec/` or `docs/` beyond decision 7's files and its declaration. A vector through the engine that
grants `new-conversation.v1` through a Conduit is not this order's. The harness's switch from its
restart to this operation is the coordinator's, once 0.2.1 is installed.

## Who decided what

FR-339 as its verifier corrected it, a separate operation not marked as model work, is the full
review's. The deferral of the operation to a release after 0.2.0 is the targeted review's
verdicts M04 and M06 with V32, and NAM-024's allowance of a further operation with its default
implementations is in `spec/` ([ruling 82](../proposals/ruling-82/README.md), point 4, written by
order 199); [ruling 81](../proposals/ruling-81/README.md#what-it-does-not-change) holds that the
review's order 62 is not held. Everything else here is the coordinator's, each open to the
owner's strike ([ruling 48](../proposals/ruling-48/README.md)): building it after 0.2.0 merges,
not on the 050 branch; the release number 0.2.1; the names `new-conversation.v1` and
`new_conversation`; the contract's `effect`, its `destructiveHint`, its empty closed input and its
output, the conversation's number within one run as the ID the log's row asks for, and `ended`;
`.v1` and `.v2` alike; moving the engine's copies of the contracts with the release, in this
order, while the generator keeps its release `agents-0.2.0` as 0.2.0's; the declaration of the
eight fixture changes, written in the build; the stop as at a failed
turn; forgetting the thread, the failure and the turn count, so that a failed conversation's
successor is allowed and turn numbers count per conversation; refusing an input; the
configuration error's answer; the lock and cancellation behavior; the tool's description; 050a's
check applied as NAM-024 bounds it; the tests, and the renamed test; and the README's wording.
Apply them as written and report them under `decisions` as the coordinator's.

## Read first

1. [FR-339](../proposals/full-review/findings/P06-dogfooding.md#fr-339), its verifier's
   counterargument and corrections; row 4-24 of
   [increment 4's log](../proposals/first-build/log-increment-4.md).
2. The targeted review's
   [verdicts M04 to M06](../proposals/review-signin-2026-11-16/verdicts.md#a-the-release-and-the-contracts-it-carries)
   and [V32](../proposals/review-signin-2026-11-16/findings.md#v32-m06-defers).
3. In `spec/`: NAM-024 and NAM-026 (`spec/promises/naming.yaml`); TPL-004
   (`spec/promises/templates.yaml`); CTR-002 (`spec/promises/contracts.yaml`); CON-017
   (`spec/promises/conduits.yaml`); IFC-025 and IFC-033
   (`spec/interface/implementation.yaml`).
4. Orders [050a](050a-agents-v2-own-tools.md) (its decision 10 and its byte check),
   [050d](050d-agents-v2-process-tree.md) and [050e](050e-agents-v2-failed-turn-fields.md), each
   with its Record, and 050n's order and, where it is on `main`, its Record.
5. Each package's `README.md`, its sections "Turn boundaries and failures (both versions)" and
   "The .v2 failed turn's fields".
6. `engine/crates/shell/tests/agents.rs`, its check of the copies and its version pin, and the
   copies' `README.md` under `conformance/fixtures/agents/`.
7. `engine/crates/conformance/examples/make_archives.rs`, where it builds the release
   `agents-0.2.0`, and
   [the install archives' README](../../conformance/fixtures/install-archives/README.md).
8. `tools/vector_delta.py --help`, [reviewing vector changes](../../conformance/README.md#reviewing-vector-changes),
   and the case `conformance/language/registration/live-agents/case.yaml`, its `files`.

**Size: M**: about 200 changed lines in each package (the two share one source): some 20 in
`lib.rs`, 35 in `main.rs`, 35 in `contracts.yaml`, 10 in `conduit.yaml`, 100 in
`tests/wrapper.rs` and 25 in the README; and, outside them, decision 7's copies of two files and a
rebuilt archive in each package's copy, with about 12 changed lines elsewhere, some 10 in
the generator and the 12-line declaration.

## Scope

Change only, in `packages/agent-codex/` and `packages/agent-claude-code/`: `Cargo.toml`,
`Cargo.lock`, `conduit.yaml`, `contracts.yaml`, `src/lib.rs`, `src/main.rs`, `tests/wrapper.rs`
and `README.md`. Outside them, decision 7's files alone: in each of
`conformance/fixtures/agents/agent-codex/` and `conformance/fixtures/agents/agent-claude-code/`,
`conduit.yaml`, `contracts.yaml`, `contracts.tar.gz` and `README.md`;
`conformance/language/registration/live-agents/fixture.json`; `engine/examples/s1/conduit.yaml`;
`engine/crates/shell/tests/agents.rs`; `engine/crates/conformance/examples/make_archives.rs`; and
`engine/STANDING.md` only as decision 7 says; and the new file
`docs/work-orders/203-declared.yaml`, which you write as decision 7 gives it. Nothing
else, in the packages or outside them. The build's ignored output (`target/`, `dist/`) may change.

## Done when

From the repository root, each passes:

```bash
python3 tools/check_docs.py
python3 tools/check_private_paths.py
python3 tools/set_names.py --check
python3 tools/promise_standing.py --check
git diff --check
```

Each prints what is said beside it:

```bash
git status --short | grep -vE '^ M (packages/agent-(codex|claude-code)/(Cargo\.toml|Cargo\.lock|conduit\.yaml|contracts\.yaml|src/lib\.rs|src/main\.rs|tests/wrapper\.rs|README\.md)|conformance/fixtures/agents/agent-(codex|claude-code)/(conduit\.yaml|contracts\.yaml|contracts\.tar\.gz|README\.md)|conformance/language/registration/live-agents/fixture\.json|engine/examples/s1/conduit\.yaml|engine/crates/shell/tests/agents\.rs|engine/crates/conformance/examples/make_archives\.rs|engine/STANDING\.md)$|^\?\? docs/work-orders/203-declared\.yaml$' | wc -l   # 0
git diff --quiet "$BASE" -- packages/agent-codex/tests/fixtures packages/agent-claude-code/tests/fixtures packages/agent-codex/src/bin packages/agent-claude-code/src/bin packages/agent-codex/tests/package.rs packages/agent-claude-code/tests/package.rs packages/agent-codex/tests/manifest.rs spec; echo $?   # 0
git diff --name-only "$BASE" -- engine conformance | grep -vxE 'conformance/fixtures/agents/agent-(codex|claude-code)/(conduit\.yaml|contracts\.yaml|contracts\.tar\.gz|README\.md)|conformance/language/registration/live-agents/fixture\.json|engine/examples/s1/conduit\.yaml|engine/crates/shell/tests/agents\.rs|engine/crates/conformance/examples/make_archives\.rs|engine/STANDING\.md' | wc -l   # 0
git diff --quiet "$BASE" -- conformance/fixtures/s1/packages/agents-0.2.0 conformance/fixtures/install-archives conformance/language/packages/agents-v2-defaults; echo $?   # 0
for NAME in agent-codex agent-claude-code; do for F in conduit.yaml contracts.yaml; do cmp -s packages/$NAME/$F conformance/fixtures/agents/$NAME/$F; echo "$NAME $F $?"; tar -xOzf conformance/fixtures/agents/$NAME/contracts.tar.gz $F | cmp -s - conformance/fixtures/agents/$NAME/$F; echo "$NAME archive $F $?"; done; done   # each 0
for NAME in agent-codex agent-claude-code; do tar -tzf conformance/fixtures/agents/$NAME/contracts.tar.gz | tr '\n' ' '; echo; done   # conduit.yaml contracts.yaml, each, in the order the archive held them at BASE
for NAME in agent-codex agent-claude-code; do tar -tvzf conformance/fixtures/agents/$NAME/contracts.tar.gz | awk '{$5 = "SIZE"; print}'; done   # the lines the start checks kept, byte for byte
for NAME in agent-codex agent-claude-code; do D=$(shasum -a 256 conformance/fixtures/agents/$NAME/contracts.tar.gz | cut -c1-64); grep -c "sha256:$D" conformance/language/registration/live-agents/fixture.json engine/examples/s1/conduit.yaml; done   # 1 for each file, for each package
git diff -U0 "$BASE" -- conformance/language/registration/live-agents/fixture.json engine/examples/s1/conduit.yaml | grep -E '^[-+][^-+]' | grep -cvE '"?(version|digest)"?:'   # 0
git diff -U0 "$BASE" -- conformance/language/registration/live-agents/fixture.json engine/examples/s1/conduit.yaml | grep -cE '^\+.*0\.2\.1'   # 4
git diff -U0 "$BASE" -- engine/crates/shell/tests/agents.rs | grep -E '^[-+][^-+]' | wc -l   # 2
grep -c 'version: 0\.2\.1\\n    digest' engine/crates/shell/tests/agents.rs   # 1
grep -c "(work order 203, the full review's order 62)" conformance/fixtures/agents/agent-codex/README.md conformance/fixtures/agents/agent-claude-code/README.md   # 1 each, where BASE's README names an order; 0 each otherwise
git diff -U0 "$BASE" -- conformance/fixtures/agents/agent-codex/README.md conformance/fixtures/agents/agent-claude-code/README.md | grep -E '^[-+][^-+]' | sed -E 's/^[-+]//; s/0\.2\.[01]/V/g; s/\(work order [^)]*\)/(O)/g' | sort | uniq -c | awk '$1 % 2' | wc -l   # 0
git diff -U0 "$BASE" -- packages/agent-codex/Cargo.toml packages/agent-claude-code/Cargo.toml packages/agent-codex/Cargo.lock packages/agent-claude-code/Cargo.lock | grep -E '^[-+][^-+]' | grep -cvE '^(-version = "0\.2\.0"|\+version = "0\.2\.1")$'   # 0
git diff -U0 "$BASE" -- packages/agent-codex/Cargo.toml packages/agent-claude-code/Cargo.toml packages/agent-codex/Cargo.lock packages/agent-claude-code/Cargo.lock | grep -cE '^\+version = "0\.2\.1"$'   # 4
git diff -U0 "$BASE" -- packages/agent-codex/conduit.yaml packages/agent-claude-code/conduit.yaml | grep -E '^-[^-]' | grep -cvE "^-(version: 0\.2\.0|    implements: \[.*'#turn\.v1'.*\])$"   # 0
diff packages/agent-codex/src/lib.rs packages/agent-claude-code/src/lib.rs | grep -c '^[<>]'   # the count recorded at BASE (2 at this writing)
diff packages/agent-codex/src/main.rs packages/agent-claude-code/src/main.rs | grep -c '^[<>]'   # 0
grep -c 'pub async fn new_conversation(&mut self) -> Result<Value>' packages/agent-codex/src/lib.rs packages/agent-claude-code/src/lib.rs   # 1 each
grep -c '"name":"new_conversation"' packages/agent-codex/src/main.rs packages/agent-claude-code/src/main.rs   # 1 each
grep -c '^## A new conversation (both versions)$' packages/agent-codex/README.md packages/agent-claude-code/README.md   # 1 each
grep -c 'a-new-conversation-both-versions' packages/agent-codex/README.md packages/agent-claude-code/README.md   # 1 each
for NAME in agent-codex agent-claude-code; do test "$(grep -c '(#the-v2-failed-turns-fields)' packages/$NAME/README.md)" -gt "$(git show "$BASE:packages/$NAME/README.md" | grep -c '(#the-v2-failed-turns-fields)')"; echo "$NAME link $?"; done   # agent-codex link 0, agent-claude-code link 0
for NAME in agent-codex agent-claude-code; do tr '\n' ' ' < packages/$NAME/README.md | grep -c 'exposes only `turn`'; done   # 0, 0
for NAME in agent-codex agent-claude-code; do tr '\n' ' ' < packages/$NAME/README.md | grep -c 'The plain MCP wrapper exposes `turn` and `new_conversation`, and each `.v2` item also `retire`; the manifest maps each versioned operation or hook to its tool\.'; done   # 1, 1
for NAME in agent-codex agent-claude-code; do test "$(grep -cE '#\[(tokio::)?test' packages/$NAME/tests/wrapper.rs)" -eq "$(( $(git show "$BASE:packages/$NAME/tests/wrapper.rs" | grep -cE '#\[(tokio::)?test') + 1 ))"; echo "$NAME tests $?"; done   # agent-codex tests 0, agent-claude-code tests 0
grep -rnE '/(Users|home)/|/(private/)?tmp/' packages/agent-codex packages/agent-claude-code --exclude-dir=target --exclude-dir=dist | wc -l   # 0
```

The vector changes, against decision 7's declaration and then as the local gate reads them (with
the interpreter that has PyYAML):

```bash
/usr/bin/python3 tools/vector_delta.py "$BASE" WORKTREE --declared docs/work-orders/203-declared.yaml; echo "exit $?"
/usr/bin/python3 tools/vector_delta.py "$BASE" WORKTREE --declared-changed | tail -n 2; echo "exit ${PIPESTATUS[0]}"
```

The first prints eight `fixture` lines of `registration-live-agents`, one for each path of the
declaration, each `"declared": true`; then
`Declarations read from docs/work-orders/203-declared.yaml: 1 shapes.`,
`Vector delta: 8 changes; 0 flips; 0 undeclared; 0 errors.` and `exit 0`. The second reads the
new file as untracked and prints the same two lines and `exit 0`. Report each last line; if any
other change appears, stop as decision 7 says.

The contracts and manifests, read as text and as YAML (with the interpreter that has PyYAML):

```bash
/usr/bin/python3 - "$BASE" <<'EOF'
import copy, re, subprocess, sys, yaml
base = sys.argv[1]
order = open("docs/work-orders/203-new-conversation.md").read()
block = order.split("```yaml\n", 1)[1].split("```", 1)[0]
def show(ref, path):
    return subprocess.run(["git", "show", f"{ref}:{path}"], capture_output=True, text=True, check=True).stdout
for name in ("agent-codex", "agent-claude-code"):
    path = f"packages/{name}/contracts.yaml"
    new, old, v011 = open(path).read(), show(base, path), show("e4ceb4c8", path)
    if not new.endswith(block):
        print(name, "operation block missing")
        continue
    head = new[: -len(block)]
    head, ops = re.subn(r", '#new-conversation\.v1'(?=\])", "", head)
    head, flow = re.subn(r", '#new-conversation\.v1': '#[a-z0-9-]+'", "", head)
    head, lines = re.subn(r"\n *'#new-conversation\.v1': '#[a-z0-9-]+'(?=\n)", "", head)
    types = [d for d in yaml.safe_load_all(new) if d and d["kind"] == "Type"]
    same = all(t["spec"]["implementation"].get("#new-conversation.v1") == t["spec"]["implementation"]["#turn.v1"] for t in types)
    print(name, "0.2.0", int(head == old), "0.1.1", int(head.startswith(v011)), "operations", ops, "entries", flow + lines, "same item", int(same and len(types) == 2))
    m, before = yaml.safe_load(open(f"packages/{name}/conduit.yaml")), yaml.safe_load(show(base, f"packages/{name}/conduit.yaml"))
    want = copy.deepcopy(before)
    want["version"] = "0.2.1"
    want["exports"].insert(want["exports"].index("turn.v1") + 1, "new-conversation.v1")
    held = [k for k, i in want["implementations"].items() if "#turn.v1" in i["implements"]]
    for k in held:
        item = want["implementations"][k]
        item["implements"].insert(item["implements"].index("#turn.v1") + 1, "#new-conversation.v1")
        item["tools"]["#new-conversation.v1"] = "new_conversation"
    print(name, "manifest", m["version"], "equal", int(m == want), len(held))
EOF
```

It prints, for each package, `NAME 0.2.0 1 0.1.1 1 operations 1 entries 2 same item 1` and
`NAME manifest 0.2.1 equal 1 K`, `K` not `0` (the items that implement `turn.v1`). The second
line applies decision 2 to `BASE`'s manifest, read as YAML (the version, the export, and each held
item's `implements` and `tools`), and finds the new manifest equal to the result, so no other item
and no other member changed. The first line is 050a's `.v1` check as NAM-024
bounds it: with exactly decision 1's additions removed, the file is 0.2.0's byte for byte, and
0.1.1's is its byte prefix.

In each of `packages/agent-codex/` and `packages/agent-claude-code/`, one command at a time:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release
cargo run --release --bin package
```

`cargo test --release` reports, for `tests/wrapper.rs`, one more passed than at `BASE` and 0
failed, and every other test target as at `BASE`. Report the totals, what
`new_conversation_shares_no_context_without_restart` printed, taken from
`cargo test --release --test wrapper new_conversation_shares_no_context -- --nocapture`, and the
digest that `package` prints: it is a host build's. Say on which system the tests ran.

Then show that the new test fails without decision 3's reset of the thread. In
`packages/agent-codex/`:

```bash
cp src/lib.rs target/lib.rs.fixed
```

Make one edit to `src/lib.rs`: delete the line `        self.thread = None;` in
`new_conversation`. Then run `cargo test --release --test wrapper new_conversation_shares_no_context`;
it must fail, in its `v1` case, at step 4's assertion that the turn's `args` hold no `resume`.
Restore with `cp target/lib.rs.fixed src/lib.rs` (not `mv`, so that cargo rebuilds), rerun the
same command, which passes, and check that `cmp target/lib.rs.fixed src/lib.rs` prints nothing.

Then run the engine's tests, which `tools/gate.py` runs for this branch and which check decision
7's copies. In `engine/`, one command at a time:

```bash
cargo clippy --all-targets -- -D warnings
cargo test --release --test agents
cargo test --release
cargo run --release -p conduit-conformance --example make_archives -- --check
```

Each test target passes with 0 failed, and the totals are those at `BASE`: decision 7 adds no
test. The last command's last line is the one the start checks kept,
`N reproducible archive fixtures verified` with the same N: the release `agents-0.2.0` and every
other generated byte are as at `BASE`.
`live_s1_scripted_agents_call_engine_with_distinct_run_identities` passes, so the copies equal the
0.2.1 package files and the pinned version and digests match. Report each command's last line
and the totals.

Then check the contracts with the engine, which the package tests cannot: that the loader accepts
the new Operation and a Conduit that lists it, and that TPL-004 finds each Type's default. Change
nothing under `engine/` for it.

- In `engine/`, run `cargo build --release`.
- In a new temporary directory T outside the repository, run
  `engine/target/release/conduit init --store T/store`.
- For each package, write a project as `engine/crates/shell/tests/agents.rs` writes one: its
  `conduit.yaml` pins the package's address at version 0.2.1 with the digest that
  `cargo run --release --bin package` printed. Its `template.yaml` holds an Agent template of the
  `.v1` Type with `accountServices: false` and `peerMessaging: false`; an Agent template of the
  `.v2` Type whose `config` holds each member that the Type's schema lists in `required` and gives
  no `default`, each set to `null`, and nothing else; and two Conduit templates of the package's
  `conversation.v1`, one listing the operations `turn.v1` and `new-conversation.v1`, the other
  `new-conversation.v1` alone, each written with the package's address as
  `engine/examples/s1/templates.yaml`'s `codex-turn` writes it.
- Run `conduit register --store T/store --package ARCHIVE#SHA256 PROJECT` for each. A project of
  templates alone starts no wrapper and no client. Each reply must be accepted with no diagnostic:
  under CON-017 a Conduit template's operations are each one its capability includes, so the
  second Conduit is accepted only because `conversation.v1` now holds `new-conversation.v1`.
- Then, for each package, register a second project whose one template is a Conduit of
  `conversation.v1` listing `reset-conversation.v1`, an operation the package does not hold. It
  must not be accepted. Report each reply's `accepted` and its diagnostics.

If a `.v2` template is refused for an input that order 050n made required, report the diagnostic
and stop: do not change the template beyond the rule above.

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark
each choice you made yourself as your own, and give the last line of every check and each count
above. Where the sandbox denies PID-scoped `ps`, the `.v2` stops read no process table, so step 3
of the new test finds `stopConfirmed` false; say so, report that test as failed for that reason,
and the reader reruns it on the Mac.

The admin then runs `python3 tools/check_private_paths.py` and `git diff --cached --check` on the
staged change. A fresh Codex verifier and a fresh read-only Opus reader read the result before it
merges ([ruling 79](../proposals/ruling-79/README.md), point 2), and the local gate,
`tools/gate.py`, runs before the merge ([ruling 80](../proposals/ruling-80/README.md)); its row
`vector changes` reads `203-declared.yaml` through `--declared-changed`, since this build's pull
request adds it.
