# Work order 203: a new conversation without restarting the engine, in each Agent package (the full review's order 62)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share: list in `vectors` each case whose files decision 7 changes, with the
promises each tests and whether it passes; you add no case. This order has network access only for
tests that bind a loopback port; use it for nothing else, add no crate, and run cargo offline, with
`CARGO_NET_OFFLINE=true` on each cargo command's own line. Run one cargo command at a time. Never
print a whole large file: use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never read or
print a key, a token or any credential. Never read or print any process's environment or
arguments, never run a command that lists every process on the machine, such as `ps -e`, `ps aux`
or `top`, and never signal a process that a test of this order did not start. Never run the real
`codex` or `claude` programs, not even for their help or version, never read their program files,
and sign in nowhere. The tests use each package's scripted clients, in temporary directories only.
Tests that bind loopback ports may fail in your sandbox; if they do, say so, and the admin reruns
them.

**Its number.** This order carries [the full review's](../proposals/full-review/orders.md) queue
row 62. It takes 203, the next number after 202 that no file uses, since 062 is the number of a
merged order of the condensation (the batch records' logs), whose Record is in
[the record](../implementation/WORK_ORDERS.md#order-062). When this order's Record is condensed,
its heading is `### Order 203`.

**How this order is written.** It states what each package must do and the tests that show it.
It gives exact text only for the contract, the diagnostics, the declaration and the cases the
tests prescribe. Where it describes code, the code is yours: write it in each package's own style,
and report each choice of yours as your own.

## What and why

**What it builds.** A caller that wants a fresh conversation with an Agent today has no operation
for it: `turn.v1` takes only `message`, a run holds one conversation, a failed turn ends it, and
nothing retires a slot. So the harness that runs Conduit's own work through Conduit
([ruling 70](../proposals/ruling-70/README.md)) restarts the engine for each fresh conversation,
and one engine can carry only one turn at a time, however many Agents its Fleet holds. This order
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

**When it is built: after 0.2.0, as 0.2.1.** The coordinator decides that this order starts from
the tree in which order 050n has joined the two lanes and published the Agent packages 0.2.0, not
on either lane before 050n:

- The targeted review of the 0.2.0 surface froze `conversation.v1` with operations `['#turn.v1']`
  for 0.2.0 (verdict M04) and deferred "a slot for a further conversation operation (the full
  review's order 62)" to a later release (M06), in
  [its verdicts](../proposals/review-signin-2026-11-16/verdicts.md#a-the-release-and-the-contracts-it-carries)
  and [its finding V32](../proposals/review-signin-2026-11-16/findings.md#v32-m06-defers). Adding
  the operation before 050n would change a member the review froze.
- NAM-024, as [order 199](199-nam-024-which-additions-count.md) worded it, makes the addition one
  a later release at the same address may make: "A further operation of a capability is one of
  them, and so is the default implementation of it that a Type whose earlier contract offers that
  capability adds to its `implementation`". So nothing is lost by waiting for the freeze.
- On a lane it would join 0.2.0's serial chain, which waits on 048b-join and the live check
  ([ruling 82](../proposals/ruling-82/README.md), points 1 and 3), and lengthen it.

The cost: until 0.2.1 is installed, the harness keeps its restart, one turn at a time on one
engine, and each restart sets every slot up again and so runs its account check again. That is the
owner's first dogfood milestone, serial turns; the second, parallel slots, comes once this order
ships ([ruling 89](../proposals/ruling-89/README.md), point 3), and IN_FORCE.md keeps A78's
orders one at a time only "until order 050 and the review's order 62". So, under the owner's lever
"Pipeline the chain" (ruling 89, point 2), this order is dispatched on order 050n's verified
round-1 branch, before 050n merges.

**Where this order starts.** Your tree starts at the commit the coordinator's dispatch note names
as `BASE`, which is `HEAD`. It holds order 050n's work in one of two ways, either of which serves:
050n has merged to `main`, and `BASE` is on `main`; or, stacked, `BASE` is the tip of 050n's
round-1 branch, `wo/050n-agent-packages-joined`, which holds 050n's build, committed, read by its
verifier and not yet by its reader or merged. You tell them apart by 050n's Record, which the
admin writes at the foot of 050n's order file at the merge: read that file from `main`, where it
is present before either base exists, since your round-1 tree may not hold it; until 050n merges
it has no `## Record` heading.
Report which base you have. Nothing below reads which it is, but for the start check's two lines
that say so. On a round-1 branch the copy of this order's file in your tree may be older than the
text on `main`; where they differ, `main`'s governs: read it with
`git show origin/main:docs/work-orders/203-new-conversation.md`.

This order was refreshed on 2026-11-19 against the two lanes' tips (`f1ede6a5` for Claude Code,
`34f4e662` for Codex) and against 050n's text as refreshed for its dispatch, before 050n was
built; the coordinator reads 050n's round-1 build before dispatching this order and revises this
text where they differ. What it expects of 050n is: both packages at version 0.2.0 in
`conduit.yaml` and `Cargo.toml`; `conversation.v1` with operations `['#turn.v1']`; each item that implements `turn.v1` marked `plain: true`
([ruling 86](../proposals/ruling-86/README.md), point 1), with an `implements` list that holds
`'#turn.v1'` and, in each `.v2` item, the core package's hooks that the lanes added (orders 050h
to 050m), each mapped in `tools`; 050a's check passing (the 0.1.1 contracts a byte prefix of each
`contracts.yaml`); and the engine's copies at 0.2.0: each package's `conduit.yaml` and
`contracts.yaml` equal to its copy under `conformance/fixtures/agents/`, each copy's
`contracts.tar.gz` holding those two copies, its digest pinned in
`conformance/language/registration/live-agents/fixture.json` and
`engine/examples/s1/conduit.yaml`, and `engine/crates/shell/tests/agents.rs` pinning version
0.2.0. The two packages' sources are no longer one source but for the `CLAUDE` constant: orders
050i to 050k changed Claude Code's, and 050l and 050m Codex's. Order 050n keeps them two (its
decision 1), moves the copies and pins to 0.2.0, has `make_archives` write the copies' archives, and builds the
S1 fixture's `.v2` Types from the copies. Its case for TPL-005 on `account-hook.v1` (its decision
6) installs a synthetic package that `make_archives` writes into
`conformance/fixtures/install-archives/`: the 0.2.0 Codex contracts under an address of its own,
one item copied from the copy's `wrapper-v2` without `account-hook.v1`, and a template of that
package's own `codex.v2`, so that the case names none of the copies; 050n may have written it as
a shell test instead. Which hooks each `.v2` item lists, and so which tools each wrapper lists, is as 050n leaves it: the Codex
lane's account and sign-in hooks (orders 050l and 050m) join only on a pass of the live check
(ruling 82, point 1). This order never assumes a list, and adds to the one it finds.

Before you change anything, check from the repository root that you find there what that gives,
and stop, change nothing and report if any check prints otherwise:

```bash
git merge-base --is-ancestor e4ceb4c8 "$BASE"; echo "main $?"
git merge-base --is-ancestor "$BASE" origin/main; echo "merged $?"
test "$(git rev-parse HEAD)" = "$(git rev-parse "$BASE")"; echo "head $?"
git ls-files --error-unmatch docs/work-orders/203-new-conversation.md >/dev/null; echo "order $?"
test -e docs/work-orders/203-declared.yaml; echo "declared $?"
git status --short | wc -l
git cat-file -e origin/main:docs/work-orders/050n-agent-packages-joined.md; echo "050n order $?"
git show origin/main:docs/work-orders/050n-agent-packages-joined.md | grep -c "^## Record"
grep -c '^version: 0.2.0$' packages/agent-codex/conduit.yaml packages/agent-claude-code/conduit.yaml
grep -c '^version = "0.2.0"$' packages/agent-codex/Cargo.toml packages/agent-claude-code/Cargo.toml
grep -c "^  operations: \['#turn.v1'\]$" packages/agent-codex/contracts.yaml packages/agent-claude-code/contracts.yaml
for NAME in agent-codex agent-claude-code; do N=$(( $(git show e4ceb4c8:packages/$NAME/contracts.yaml | wc -c) )); head -c "$N" packages/$NAME/contracts.yaml | cmp -s - <(git show e4ceb4c8:packages/$NAME/contracts.yaml); echo "$NAME 0.1.1 $?"; done
grep -rlE 'new-conversation|new_conversation' packages/agent-codex packages/agent-claude-code --exclude-dir=target --exclude-dir=dist | wc -l
for NAME in agent-codex agent-claude-code; do for F in conduit.yaml contracts.yaml; do cmp -s packages/$NAME/$F conformance/fixtures/agents/$NAME/$F; echo "$NAME $F $?"; tar -xOzf conformance/fixtures/agents/$NAME/contracts.tar.gz $F | cmp -s - conformance/fixtures/agents/$NAME/$F; echo "$NAME archive $F $?"; done; done
for NAME in agent-codex agent-claude-code; do D=$(shasum -a 256 conformance/fixtures/agents/$NAME/contracts.tar.gz | cut -c1-64); grep -c "sha256:$D" conformance/language/registration/live-agents/fixture.json engine/examples/s1/conduit.yaml; done
grep -c 'version: 0\.2\.0\\n    digest' engine/crates/shell/tests/agents.rs
for NAME in agent-codex agent-claude-code; do python3 -c 'import sys,tarfile; [print(m.name, oct(m.mode), m.uid, m.gid, m.uname, m.gname, m.mtime) for m in tarfile.open(sys.argv[1])]' conformance/fixtures/agents/$NAME/contracts.tar.gz; done
git grep -lE 'fixtures/agents/agent-(codex|claude-code)/|live-agents/fixture\.json|examples/s1/conduit\.yaml' -- 'conformance/**/case.yaml' 'conformance/**/case.json' 'conformance/**/case.yml'
ls conformance/fixtures/install-archives
git grep -l TPL-005 -- 'conformance/**/case.yaml' engine/crates/shell/tests | xargs grep -l 'account-hook.v1'
(cd engine && CARGO_NET_OFFLINE=true cargo run --release -p conduit-conformance --example make_archives -- --check 2>&1 | tail -n 1)
```

They print `main 0`; then `merged 0` where 050n has merged and `BASE` is on `main`, or `merged 1`
on 050n's round-1 branch; `head 0`, `order 0`, `declared 1` (no declaration yet: you write it,
decision 7) and `0`; `050n order 0` (050n's order file, on `main`), then `1` where 050n has
merged or `0` where it has not, as on its round-1 branch, since its file has no Record yet: stop where this line and the `merged` line disagree, `merged 0`
with `0` or `merged 1` with `1`; `1` for each of the two `conduit.yaml`, the two `Cargo.toml` and
the two `contracts.yaml`; `agent-codex 0.1.1 0` and `agent-claude-code 0.1.1 0` (050a's check,
against the 0.1.1 contracts as `main` held them at `e4ceb4c8`); `0` (no file holds the new names
yet); for each package and each of `conduit.yaml` and `contracts.yaml`, `NAME FILE 0` and
`NAME archive FILE 0`; for each package, `1` or more for `fixture.json` and `1` or more for
`conduit.yaml` (the archive's digest is pinned in both; keep the counts, as order 050n's done-when
allows more than one); `1` (the test's version pin); for each package, one line for each of the
archive's two members, `conduit.yaml` and `contracts.yaml`, with the name, mode, owner and group
IDs and names, and time that order 050n built it with, and no size (keep them: the done-when
compares the rebuilt archives' listings with them; the listing is Python's `tarfile`, not
`tar -tv`, whose fields differ between BSD and GNU tar); the case files that name any of decision
7's eight files as a fixture alias, among them
`conformance/language/registration/live-agents/case.yaml` (keep the list: decision 7's declaration
covers each; for each other case, keep also the files it names that pin a copy's version `0.2.0`
or a copy's digest, such as an `install-archive` step's install file); the files under
`conformance/fixtures/install-archives/` (keep the list, and name among them the synthetic TPL-005
package); the `case.yaml` of order 050n's TPL-005 case for `account-hook.v1`, or the shell test
where 050n wrote it as one (for a case, list then that `case.yaml`'s `files` entries, and name
among them its template file and its install file; keep them; that case must not
be among the cases listed above, since its template is of the synthetic package's own `codex.v2`
and it installs no copy: where it is, or where its template is of the copy's `codex.v2`, stop and
report, since what that case becomes at 0.2.1 is then the coordinator's); and
`N reproducible archive fixtures verified` (keep its line). `/usr/bin/python3` is the Mac's
interpreter with PyYAML; where `python3` has PyYAML, it serves as well.

Then, still before any change, record for the report: the tools that each wrapper lists at
`BASE`, for `.v1` and for `.v2`, in their order; and each test in each `tests/wrapper.rs` at
`BASE` that asserts the listed tools. And, in each package, run `cargo run --release --bin package`
and copy the archive it names to a new temporary directory outside the repository, keeping the
`ARCHIVE#SHA256` line it printed: the done-when installs this 0.2.0 archive beside the new one.
Record also each package's per-target test totals at `BASE` and those of the engine's target
`agents`, taking them from the coordinator's dispatch note, which carries them from 050n's
verified report, or from 050n's Record, and only for a total that neither gives by running
`cargo test --release` in that package or `cargo test --release --test agents` in `engine/`: the
done-when compares the totals with these.

**What the tree already does, so that you do not redo it.** In each package, as orders 050a to
050n leave it:

- `Session` keeps the running client, where one runs; Codex's thread ID, which later turns resume
  (Claude Code never sets it); the turn number and cause of a failed turn, after which every turn
  is refused with `session failed at turn K: CAUSE; implicit restart refused`; the turns this
  conversation has started (a failed turn's number comes from it, and the client tool report is
  `reported` and `current` only on the first); 050h's retired mark, set once the retire hook
  begins, after which every turn is refused; and, in a `.v2` Agent, the login that the sign-in
  hook started, if any.
- `Session::shutdown` drops the client's stdin, stops the wrapper's process tree for `.v2` (order
  050d's stop, which does not close the tree, so a later launch is allowed), kills and reaps the
  client, and returns 050e's `StopReport`. A failed turn's end and a cancellation use it. The
  tree holds every process group the wrapper started in the Agent's home, a running login's
  included, so this whole-tree stop also ends a login in progress.
- `Session::turn` launches a client where none runs: for Codex, `resume THREAD` where a thread is
  set, and one process per turn; for Claude Code, one process for the whole conversation, run
  with `--no-session-persistence`.
- `main.rs`'s `list_tools` lists `turn` first, then, for a `.v2` wrapper, the hooks' tools, and
  `call_tool` serves each by name and refuses any other name as invalid parameters
  (`unknown tool`). Each hook's tool refuses any argument with `expected no arguments`, waits for
  the session, and once it holds the session is not cut by a cancellation.
- The scripted client answers a message such as `TEST` with a JSON text of its evidence, holding
  among others `args`, `pid` and `number`: for Codex, `2` where it was resumed and `1` otherwise;
  for Claude Code, the line's index in that client process, from `1`. `FAIL:TEXT` fails the turn.

## The decisions it applies

**1. The contract.** In each package's `contracts.yaml`, make exactly these three changes and no
other:

- In the Capability `conversation.v1`, the line `  operations: ['#turn.v1']` becomes
  `  operations: ['#turn.v1', '#new-conversation.v1']`.
- In each Type's `spec.implementation` (the `.v1` Type and the `.v2` Type, so two in each
  package), add the key `'#new-conversation.v1'` with the same value as that map's `'#turn.v1'`
  key, right after the `'#turn.v1'` entry and in the map's own style: in a block map, a line with
  the `'#turn.v1'` line's indent, `'#new-conversation.v1': '#ITEM'`; in a flow map,
  `, '#new-conversation.v1': '#ITEM'` right after `'#turn.v1': '#ITEM'`. TPL-004 needs this
  default in every Type that offers `conversation.v1`, the `.v1` Types included (V32).
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
    one: the next turn.v1 starts the client with no conversation of the turns before it: it
    resumes no thread or session. Files in the Agent's directory and home stay. The Agent's run
    goes on.
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
fails, first at the `.v1` Type's new default-implementation line and then at `conversation.v1`'s
operations line, by design: V32 records that the `.v1` contracts change in
the release that adds the operation, "which 050a decision 10's byte-for-byte rule (a 0.2.0
decision) does not govern".

**2. The manifest and the version.** In each package's `conduit.yaml`: `version: 0.2.0` becomes
`version: 0.2.1`; in `exports`, add `- new-conversation.v1` right after `- turn.v1`; and in each
item whose `implements` list holds `'#turn.v1'`, add `'#new-conversation.v1'` to that list right
after `'#turn.v1'`, in the list's own style (flow or block), and add to its `tools` map, right
after the `'#turn.v1'` entry and with its indent, `'#new-conversation.v1': new-conversation`. No
other item and no other member changes: each item stays `plain: true`, so the new operation's
calls are plain, as the turn's are, and nothing is keyed (ruling 86, point 1); and `agent-codex`'s
`wrapper-v2-path` still differs from `wrapper-v2` only in its program (the manifest test). In
each `Cargo.toml`, `version = "0.2.0"` becomes `version = "0.2.1"`, and `Cargo.lock` changes only
in the package's own `version` line, as cargo rewrites it.

**3. The session.** In each `src/lib.rs`, give `Session` a public method that serves
`new-conversation.v1`, the same in behavior in both packages:

- **A configuration error** answers as a turn does, with the same text, and changes nothing.
- **After the retire hook has begun**, it is refused with `session retired; no new conversation starts`,
  and changes nothing: a retired Agent's turns stay refused, whatever a caller asks.
- **While a login that the sign-in hook started is still running** (a `.v2` Agent), it is refused
  with
  `new conversation refused: a sign-in is in progress in this Agent's own home; nothing was stopped; call again once the sign-in has ended`,
  and changes nothing: the login, its process and the conversation are as before. A login whose
  process has ended, completed or not, does not refuse it.
- **Otherwise** it ends the conversation with the same stop as a failed turn's end (`shutdown`,
  unchanged), forgets the conversation's thread, its failure and its turn count, and counts the
  conversation: the run's first conversation is 1, and each successful call starts the next. It
  launches no client: the next turn does. It answers
  `{"conversation":N,"ended":{"turns":T,"failed":F,"survivors":[...],"stopConfirmed":B}}`, where
  N is the conversation now started; T the turns the ended conversation started; F whether one of
  them failed or was cancelled; and the survivors and whether the stop was confirmed, from the
  stop's report as 050e's failed-turn fields give them (for `.v1`, always none and true).
- **A stop that leaves survivors or cannot be confirmed** does not refuse the call, unlike the
  retire and sign-out hooks, which refuse on the stop's note: the call succeeds, counts the next
  conversation, and reports the survivors and `stopConfirmed` false in `ended`, and the caller
  decides; the next turn launches its client whatever survived. So a caller that receives a
  non-empty `survivors` or `stopConfirmed` false should send no next turn, since the ended
  conversation's processes may still run beside it, and should end the Agent's run instead, as
  the harness's restart does before 0.2.1.

`Session::turn`, `cancel`, `shutdown` and the failed turn's end do not change. So a turn's number,
in its failure text and in 050e's fields, counts from 1 in each conversation, and the first turn
of a new conversation reports its client tools as `reported` and `current`. A failed conversation
stays ended: its later turns are refused as before, until a call to `new-conversation.v1` starts
the next one, which is an explicit request, not an implicit restart.

That a new conversation may follow a failed one is the coordinator's reading, open to the owner's
strike under [ruling 82](../proposals/ruling-82/README.md), point 5, which struck
[D55](../proposals/full-review/decisions.md#dogfooding-and-the-evidence-kept-in-the-tree)'s
un-refusing of a failed conversation for `.v2`; the coordinator puts it to the owner before this
order is dispatched, so that the answer is known before the build. The reading: this is not D55's un-refusing, since the failed conversation stays
ended and the `.v2` description's "ends the conversation" holds; and
[ruling 71](../proposals/ruling-71/README.md), point 4, does not bar it, since the S1 rerun
measures the build pinned at milestone 1 ([ruling 89](../proposals/ruling-89/README.md),
point 7), before 0.2.1, and this order merges only after that pin. If the owner strikes it, the
call is refused while the conversation's failure is set, steps 5 to 8 of decision 5's new
test change to match, and the merge waits for no pin; the coordinator revises this text before
dispatch.

Nothing of the Agent's own home changes: its path for a given configuration, what the wrapper
prepares in it, and what the hooks do in it. So a home that a 0.2.0 wrapper signed in is the home
that the 0.2.1 wrapper uses for the same configuration and state directory. Whether an installed
Agent keeps its instance, and so its state directory and its sign-in, across the move of its pin
to 0.2.1 is the engine's, not the wrapper's; nothing here promises that a 0.2.0 sign-in carries
over.

**4. The tool.** In each `src/main.rs`, the wrapper serves `new-conversation.v1` as the MCP tool
`new-conversation`, hyphenated as `sign-in` and `sign-out` are, for `.v1` and `.v2` alike:

- `list_tools` lists it last, after every tool it lists at `BASE`, which keep their order. Its
  input schema is `{"type":"object","properties":{},"additionalProperties":false}`, and its
  description, the same in both packages, says that it explicitly ends this Agent's conversation
  and starts a new one, that the next turn resumes no conversation of the turns before while
  files in the Agent's directory and home stay, and that it starts no model work.
- `call_tool` refuses any argument with invalid parameters, `expected no arguments`, before it
  touches the session. Like a turn, the call waits for the session, so a call queued behind a
  running turn or hook waits for it; a cancellation before it holds the session answers an error
  with `new conversation cancelled` and changes nothing; once it holds the session it runs to its
  end, which 050d's bounded stop keeps short. Where the wrapper's TERM stop has begun, the call
  waits for the wrapper's exit, as 050d's `stop(false)` does.
- A success has `isError` false, a complete result type, `structuredContent` the session's
  answer, and one text block holding the same JSON. A refusal or a configuration error has
  `isError` true and one text block, the text of decision 3.

Format with `cargo fmt`.

**5. The tests**, in each package's `tests/wrapper.rs`, the same cases in both:

- **`stdio_exposes_only_turn_and_setup_never_launches_client`** is renamed
  `stdio_exposes_turn_and_new_conversation_and_setup_never_launches_client`. It asserts the tools
  that `BASE` lists, in their order, then `new-conversation`, and that the last one's input
  schema, as JSON, equals `{"type":"object","properties":{},"additionalProperties":false}`. Right
  after its check that `client-started` does not exist, it calls `new-conversation` with
  `{"reset":true}`, which must be an error, then with `{}`, whose result has `isError` false and
  `structuredContent`
  `{"conversation":2,"ended":{"turns":0,"failed":false,"survivors":[],"stopConfirmed":true}}`, and
  checks again that `client-started` does not exist. The rest of the test does not change.
- **Every other test at `BASE` that asserts the listed tools** asserts them with
  `new-conversation` after them, and nothing else in it changes; report each such test.
- **`stdio_v2_defaults_and_invalid_configuration_remain_callable`**: in each case with an invalid
  configuration, after its turns, a `new-conversation` call with `{}` answers `isError` true with
  one text block, equal to the turns' `invalid configuration: ...` text.
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
  3. `new-conversation` `{}` succeeds: `isError` false, `structuredContent`
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
  7. `new-conversation` `{}` succeeds with `structuredContent`
     `{"conversation":3,"ended":{"turns":2,"failed":true,"survivors":[],"stopConfirmed":true}}`.
  8. `turn` `TEST` succeeds with `number` `1`, and for Codex `args` that hold no `resume`.
  9. `new-conversation` with `{"reset":true}` is an error, and a following `turn` `TEST`
     succeeds with `number` `2`: the refused call changed nothing.

  The test `eprintln!`s each `new-conversation` result and each turn's `number` and, for Claude
  Code, `pid`. At its end it cancels the service, and the wrapper's stderr holds none of
  `sentinel-secret`, `synthetic-test-token`, `synthetic-anthropic-key` and
  `synthetic-openai-key`.
- **`v2_retire_tool_through_the_wrapper`**: after each retire call, the failed logout's included,
  a `new-conversation` call with `{}` answers `isError` true with one block, `session retired; no new conversation starts`,
  and a `turn` `TEST` after it is still refused with the retired refusal that `BASE` gives.
- **A new test, `v2_new_conversation_keeps_a_sign_in_in_progress`**, in each package whose `.v2`
  item has a sign-in hook at `BASE`, through the `Session` API,
  with the scripted client: it starts the browser login as `BASE`'s sign-in tests of that package
  start it, holding the login port's lock where they hold one, and while the login runs calls `new-conversation`, which is refused with decision 3's
  sign-in text; the login's process is still running, and, where `BASE`'s tests can ask, the
  login still answers as one in progress. Then it ends the login as `BASE`'s sign-in tests do
  (completing it, or letting its scripted client exit), and `new-conversation` succeeds with
  `conversation` `2` and `ended.turns` `0`. It ends with `shutdown`, and no process it started
  outlives it. The two packages' sign-in APIs differ, so the test's steps may differ between them,
  but not what it shows.
- **A new test, `v2_new_conversation_reports_survivors`**. A survivor cannot be planted at `BASE`:
  the stop kills every process it finds, a process it does not find it does not report, and
  `tests/fixtures/client.rs` does not change (decision 8). So decision 3's method builds `ended`
  through one public function, from a `StopReport` with the ended conversation's turns and
  whether one failed to the `ended` JSON. The test, in `tests/wrapper.rs`, calls it with a
  `StopReport { survivors: [..], unconfirmed: true }` built directly, and asserts that
  `survivors` holds those PIDs and `stopConfirmed` is false. Report the function as your own
  choice.

Decision 4's waiting behind a running turn or hook and its cancellation before the call holds the
session are left untested by choice. Every other test keeps passing unchanged.

**6. The READMEs**, both, in the package's own client's terms. Write no absolute local path and no
client version.

- The opening paragraph that says what the manifest exports and which tools the wrapper exposes
  says that the conversation capability has two operations, the explicit `turn.v1` and
  `new-conversation.v1`, and that the wrapper exposes `turn` and `new-conversation`, the `.v2`
  item also the hooks' tools.
- The sentence "K counts turns from 1 and continues across resumes." also says that a new
  conversation counts from 1 again, with "new conversation" a link to the new section's anchor,
  `a-new-conversation-both-versions`.
- Add, at the end, the section `## A new conversation (both versions)`. It says: from 0.2.1 the
  capability `conversation.v1` holds a second operation, `new-conversation.v1`, which the wrapper
  exposes as the tool `new-conversation`; it takes no input and starts no model work, so a Conduit
  can grant turns without it, or it without turns; a call ends the Agent's conversation, stopping
  the client as at a failed turn (for `.v2`, its whole process tree) and forgetting the
  conversation's thread, its failure and its turn count, and launches no client; the next `turn`
  then resumes no conversation of the turns before, though files in the Agent's directory and home
  stay, with anything the client itself keeps there: for `agent-codex`, because it launches Codex with
  no `resume`, so Codex starts a new thread; for `agent-claude-code`, because it launches a new
  Claude Code process, and since the wrapper runs Claude Code with `--no-session-persistence`,
  nothing of the ended conversation is kept to resume; the Agent's run and the engine go on, so a
  fresh conversation needs no restart; after a failed turn the failed conversation stays ended and
  its later turns are refused, and `new-conversation` starts the next, a new conversation, not the
failed one resumed. Then the result, with N, T,
  F and the stop's fields as decision 3 gives them, N counting within one run of the wrapper only
  (a new run counts from 1 again), and a link to the failed turn's fields, the anchor
  `the-v2-failed-turns-fields`, for the survivors and the stop's confirmation, and that a caller
  that receives a non-empty `survivors` or `stopConfirmed` false should send no next turn, since
  the ended conversation's processes may still run beside it, and should end the Agent's run
  instead. Then each refusal, with its text: an input, as invalid parameters; an invalid
  configuration, as a turn answers it; a retired Agent; and, for `.v2`, a sign-in in progress,
  which a new conversation never stops. And that through a Conduit, an input is rejected by the
  engine as invalid, with no outcome, before the wrapper is called; the wrapper's own refusals
  (configuration, retired, sign-in in progress) are outcome unknown with its text, as a turn's
  failure is, since the contract states an `effect` and the item is plain (IFC-010), so a caller
  retries with a fresh idempotency key. And that a call
  waits for a running turn or hook. And: the
  0.2.1 contracts are 0.2.0's with NAM-024's additions alone, the operation in
  `conversation.v1`'s list, its Operation contract and each Type's default implementation of it,
  the `.v1` Types included; and the Agent's own home is as in 0.2.0 for the same configuration and
  state directory, while whether an installed Agent keeps its instance, and so its sign-in, across
  the move to 0.2.1 is the engine's, so the README does not promise that a 0.2.0 sign-in carries
  over.

**7. The engine's copies.** The engine's test `engine/crates/shell/tests/agents.rs`, which
`tools/gate.py` runs for any change under the two packages, requires each package's
`conduit.yaml` and `contracts.yaml` to equal its copy under `conformance/fixtures/agents/` byte
for byte, and registers an archive built from the package under a pinned version (PKG-007 refuses
a pin whose version or digest does not match). The generator of the install-archive fixtures,
`make_archives`, builds its release `agents-0.2.0` from one of those copies, and its `--check`
compares every byte it would write. So the copies move to 0.2.1 with the packages, and the
generator keeps 0.2.0's release as it is:

- In each of `conformance/fixtures/agents/agent-codex/` and
  `conformance/fixtures/agents/agent-claude-code/`: `conduit.yaml` and `contracts.yaml` become
  byte-for-byte copies of the package's new files; `contracts.tar.gz` is rebuilt from those two
  copies alone, deterministically, with the listing that the start checks kept (the same member
  names in the same order, with the same modes, owner, group and time), its size apart; and `README.md` names version
  0.2.1 where it named 0.2.0 and, where it names the order that made the copies, names
  `(work order 203, the full review's order 62)`, with no other change. Order 050n has
  `make_archives` write the copies' archives (its decision 4): rebuild them by running it in write
  mode (`cargo run --release -p conduit-conformance --example make_archives` in `engine/`), which,
  with the next bullet's change, leaves every other output it writes at its `BASE` bytes. Where
  `BASE`'s `make_archives` does not write them, build them as 050n's decision 4 states them, and
  report how, as your own choice. Where `make_archives` also lists the copies' archives in
  `conformance/fixtures/install-archives/` and its `digests.json`, those entries, whose new bytes
  equal a copy's new `contracts.tar.gz`, are the only change allowed there.
- In `engine/crates/conformance/examples/make_archives.rs`, the release `agents-0.2.0`, which
  takes documents from the copies (the non-Type documents of the Claude Code copy's
  `contracts.yaml` and, as order 050n left it, members of the `.v2` Types), and every other
  generator output built from the copies, the synthetic TPL-005 package that order 050n has
  `make_archives` write from the Codex copy included, each keep their `BASE` bytes:
  none holds `new-conversation.v1`, neither the Operation, nor its entry in
  `conversation.v1`'s `operations`, nor a Type's default implementation of it, nor an item's
  `implements` entry or `tools` entry for it. How the generator leaves them out is yours to choose
  and report. A fixture named 0.2.0 does not hold 0.2.1's operation, and so
  `conformance/fixtures/s1/packages/agents-0.2.0`, its archive and the synthetic package's under
  `conformance/fixtures/install-archives/`, `digests.json` (but for the entries the bullet above
  allows) and the `agents-v2-defaults` vector's pinned digest stay byte for
  byte: the write mode leaves them as they are. Nothing else in the generator changes.
- In `conformance/language/registration/live-agents/fixture.json` and
  `engine/examples/s1/conduit.yaml`: for each Agent package, its `version` becomes `0.2.1` and its
  `digest` the `sha256:` digest of the copy's new `contracts.tar.gz`. Nothing else changes.
- In `engine/crates/shell/tests/agents.rs`: the pin `version: 0.2.0` becomes `version: 0.2.1`.
  Nothing else changes.
- In each other case that the start checks listed, such as order 050n's portable cases that
  register each package's copy, the files the start checks kept as pinning a copy's version or
  digest (an `install-archive` step's install file, say): the copy's `version` `0.2.0` becomes
  `0.2.1` and its digest the copy's new one, in that file's own form. Nothing else in them
  changes, and no `case.yaml` changes; a case whose name or title says 0.2.0 keeps it, and you
  report it.
- **The TPL-005 case.** Order 050n's case for TPL-005 on `account-hook.v1` installs the
  synthetic package, whose template is of its own `codex.v2` and whose bytes stay as at `BASE`
  (the bullet above), so it passes unchanged and is not in the declaration. Where 050n wrote it as
  a shell test, that test does not change either.
- **The declaration.** The case `registration-live-agents` names eight of these files as fixture
  aliases: `fixture.json`, `engine/examples/s1/conduit.yaml` and, for each package, the copy's
  `conduit.yaml`, `contracts.yaml` and `contracts.tar.gz`. Order 050n may have added cases that
  name some of them too: the start checks listed every such case. `tools/vector_delta.py` compares each
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

  For each other case that the start checks listed, add after it one shape of the same form,
  named `the copies of the Agent packages move to 0.2.1, in CASE`, its `cases` that case's ID and
  its `paths` the files of these eight that the case names, in the order above, then the files of
  that case whose pins the item above moves. Change the file only to match
  the change: if `vector_delta.py` reports a fixture change outside those shapes, a change of any case the start
  checks did not list, or any `case.yaml` change, stop and report it; do not widen a shape. If a
  case that the start checks listed needs any other change to pass with the 0.2.1 copies, its
  `case.yaml` or an expectation beyond those pins, stop and report it: the coordinator decides,
  with this order's text.

No vector's `case.yaml`, no promise and no other test changes, beyond the pins above; if the
engine's tests need one to
change, stop and report. Then run `python3 tools/promise_standing.py --check`; if it reports
`engine/STANDING.md` stale, run it with `--write`, which may change that file alone, and report
the change.

**8. What this order does not do.** No change to `turn.v1`, to any description, or to any
`schema`, `offers`, `requires`, `signIn` or `records`; no input on `turn.v1`; no continuing of a
named conversation; no conversation ID in a turn's output; no `keyed` on any item (ruling 86);
no change to the Agent's own home or to any hook's behavior; no change to `tests/fixtures/client.rs`,
`src/bin/package.rs`, `tests/package.rs` or `tests/manifest.rs`; and none under `engine/`,
`conformance/`, `spec/` or `docs/` beyond decision 7's files and its declaration. A vector through
the engine that grants `new-conversation.v1` through a Conduit is not this order's. The harness's
switch from its restart to this operation, and the Fleet's Conduits that grant it, are the
coordinator's, once 0.2.1 is installed. So are `engine/CONFORMANCE.md`'s present-state bullet and
`docs/guides/accounts.md`, which order 050n brings to 0.2.0: the coordinator brings them to 0.2.1,
with one sentence on `new-conversation.v1`, in this order's merge; report where each names 0.2.0.

## Who decided what

FR-339 as its verifier corrected it, a separate operation not marked as model work, is the full
review's. The deferral of the operation to a release after 0.2.0 is the targeted review's
verdicts M04 and M06 with V32, and NAM-024's allowance of a further operation with its default
implementations is in `spec/` ([ruling 82](../proposals/ruling-82/README.md), point 4, written by
order 199); [ruling 81](../proposals/ruling-81/README.md#what-it-does-not-change) holds that the
review's order 62 is not held. The items stay plain, as ruling 86, point 1, has them. Everything
else here is the coordinator's, each open to the owner's strike
([ruling 48](../proposals/ruling-48/README.md)), but for its place: that order 62 comes next on
the path after order 050 is the owner's, [ruling 80](../proposals/ruling-80/README.md), point 7,
which supersedes the review's "After" the pin; that parallel slots, the second dogfood
milestone, come once this order ships is the owner's ([ruling 89](../proposals/ruling-89/README.md),
point 3), a target that sets no gate and no deadline by the coordinator's reading; and the lever
"Pipeline the chain" is the owner's (ruling 89, point 2). The coordinator's: building it after
0.2.0, not on a lane; applying that lever to this order, which is not on the sign-in chain, so that
it starts from 050n's round-1 branch, and the start check and the steps at the merge that go with
it; the release number 0.2.1; the names `new-conversation.v1` and
`new-conversation`; the contract's `effect`, its `destructiveHint`, its empty closed input and its
output, the conversation's number within one run as the ID the log's row asks for, and `ended`;
`.v1` and `.v2` alike; moving the engine's copies of the contracts with the release, in this
order, while the generator keeps its release `agents-0.2.0` as 0.2.0's; the declaration of the
eight fixture changes, written in the build; the stop as at a failed turn; forgetting the thread,
the failure and the turn count, so that a failed conversation's successor is allowed and turn
numbers count per conversation, the successor by the coordinator's reading in decision 3, open to
the owner's strike under ruling 82, point 5, and put to the owner before dispatch, since the full
review's verifier held un-poisoning a failed conversation back from the coordinator before the S1
rerun (ruling 71, point 4); the gate that this order merges only after the S1 rerun's pin, open
to the owner's strike, which follows from that reading and narrows ruling 80, point 7's placement
only if the pin lags milestone 1, and the coordinator tells the owner if the gate holds this order
past milestone 1; the advice to a caller on survivors; refusing an input; the configuration error's answer; through a
Conduit, an input is rejected by the engine as invalid, with no outcome, before the wrapper is
called, and the wrapper's own refusals (configuration, retired, sign-in in progress) are outcome
unknown with its text, as a turn's failure is, since the contract states an `effect` and the item
is plain (IFC-010), so a caller retries with a fresh idempotency key; a stop that leaves survivors or is not confirmed succeeds and reports them;
refusing a
new conversation after the retire hook has begun, and while a sign-in is in progress, so that a
caller granted only this operation can neither end a sign-in nor revive a retired Agent; the
refusals' texts; the home kept as in 0.2.0; the lock and cancellation behavior; the tool's
description; 050a's check applied as NAM-024 bounds it; the tests, and the renamed test; and the
READMEs' content. Apply them as written and report them under `decisions` as the coordinator's.

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
   (`spec/interface/implementation.yaml`); PKG-020 (`spec/promises/packages.yaml`), for `plain`.
4. Orders [050a](050a-agents-v2-own-tools.md) (its decision 10 and its byte check),
   [050d](050d-agents-v2-process-tree.md), [050e](050e-agents-v2-failed-turn-fields.md) and
   [050h](050h-agents-v2-retire-hook.md); the sign-in hooks of orders
   [050j](050j-claude-sign-in-redirect-login.md) and [050m](050m-codex-sign-in-hook.md); each
   with its Record; and 050n's order and, where it is on `main`, its Record.
5. Each package's `README.md`, its sections "Turn boundaries and failures (both versions)", "The
   .v2 failed turn's fields", "The .v2 retire hook" and "The .v2 sign-in hook".
6. `engine/crates/shell/tests/agents.rs`, its check of the copies and its version pin, and the
   copies' `README.md` under `conformance/fixtures/agents/`.
7. `engine/crates/conformance/examples/make_archives.rs`, where it builds the release
   `agents-0.2.0`, and
   [the install archives' README](../../conformance/fixtures/install-archives/README.md).
8. `tools/vector_delta.py --help`, [reviewing vector changes](../../conformance/README.md#reviewing-vector-changes),
   and the case `conformance/language/registration/live-agents/case.yaml`, its `files`.

**Size: M**: about 250 changed lines in each package: some 30 in `lib.rs`, 40 in `main.rs`, 35
in `contracts.yaml`, 10 in `conduit.yaml`, 130 in `tests/wrapper.rs` and 25 in the README; and,
outside them, decision 7's copies of two files and a rebuilt archive in each package's copy, with
about 25 changed lines elsewhere, the generator and the 12-line declaration among them.

## Scope

Change only, in `packages/agent-codex/` and `packages/agent-claude-code/`: `Cargo.toml`,
`Cargo.lock`, `conduit.yaml`, `contracts.yaml`, `src/lib.rs`, `src/main.rs`, `tests/wrapper.rs`
and `README.md`; and, in `packages/agent-claude-code/`, `src/sign_in.rs`, only where decision 3's
check of a running login needs it. Outside them, decision 7's files alone: in each of
`conformance/fixtures/agents/agent-codex/` and `conformance/fixtures/agents/agent-claude-code/`,
`conduit.yaml`, `contracts.yaml`, `contracts.tar.gz` and `README.md`;
`conformance/language/registration/live-agents/fixture.json`; `engine/examples/s1/conduit.yaml`;
`engine/crates/shell/tests/agents.rs`; `engine/crates/conformance/examples/make_archives.rs`; the
files of the other cases the start checks listed whose pins decision 7 moves;
in `conformance/fixtures/install-archives/` and its `digests.json`, the copies' archives and their
entries alone, where decision 7 allows them; `engine/STANDING.md` only as decision 7 says; and the
new file `docs/work-orders/203-declared.yaml`, which you write as decision 7 gives it. Nothing else, in the
packages or outside them. The build's ignored output (`target/`, `dist/`) may change.

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
git status --short | grep -vE '^ M (packages/agent-(codex|claude-code)/(Cargo\.toml|Cargo\.lock|conduit\.yaml|contracts\.yaml|src/lib\.rs|src/main\.rs|tests/wrapper\.rs|README\.md)|packages/agent-claude-code/src/sign_in\.rs|conformance/fixtures/agents/agent-(codex|claude-code)/(conduit\.yaml|contracts\.yaml|contracts\.tar\.gz|README\.md)|conformance/language/registration/live-agents/fixture\.json|engine/examples/s1/conduit\.yaml|engine/crates/shell/tests/agents\.rs|engine/crates/conformance/examples/make_archives\.rs|conformance/fixtures/install-archives/(digests\.json|[^/]+\.tar\.gz)|engine/STANDING\.md)$|^\?\? docs/work-orders/203-declared\.yaml$' | grep -vxF "$(sed -n 's/^  - / M /p' docs/work-orders/203-declared.yaml)" | wc -l   # 0
git diff --quiet "$BASE" -- packages/agent-codex/tests/fixtures packages/agent-claude-code/tests/fixtures packages/agent-codex/src/bin packages/agent-claude-code/src/bin packages/agent-codex/tests/package.rs packages/agent-claude-code/tests/package.rs packages/agent-codex/tests/manifest.rs spec; echo $?   # 0
git diff --name-only "$BASE" -- conformance/fixtures/s1/packages/agents-0.2.0 conformance/language/packages/agents-v2-defaults | wc -l   # 0
for F in $(git diff --name-only "$BASE" -- conformance/fixtures/install-archives ':(exclude)conformance/fixtures/install-archives/digests.json'); do cmp -s "$F" conformance/fixtures/agents/agent-codex/contracts.tar.gz || cmp -s "$F" conformance/fixtures/agents/agent-claude-code/contracts.tar.gz; echo "$F $?"; done   # nothing, or each 0: only the copies' archives, where make_archives lists them there
git diff "$BASE" -- conformance/fixtures/install-archives/digests.json   # nothing, or only the entries decision 7 allows
ls conformance/fixtures/install-archives   # the names the start checks kept
for NAME in agent-codex agent-claude-code; do for F in conduit.yaml contracts.yaml; do cmp -s packages/$NAME/$F conformance/fixtures/agents/$NAME/$F; echo "$NAME $F $?"; tar -xOzf conformance/fixtures/agents/$NAME/contracts.tar.gz $F | cmp -s - conformance/fixtures/agents/$NAME/$F; echo "$NAME archive $F $?"; done; done   # each 0
for NAME in agent-codex agent-claude-code; do python3 -c 'import sys,tarfile; [print(m.name, oct(m.mode), m.uid, m.gid, m.uname, m.gname, m.mtime) for m in tarfile.open(sys.argv[1])]' conformance/fixtures/agents/$NAME/contracts.tar.gz; done   # the lines the start checks kept, byte for byte
for NAME in agent-codex agent-claude-code; do D=$(shasum -a 256 conformance/fixtures/agents/$NAME/contracts.tar.gz | cut -c1-64); grep -c "sha256:$D" conformance/language/registration/live-agents/fixture.json engine/examples/s1/conduit.yaml; done   # the counts the start checks kept, for each file and package
grep -c 'version: 0\.2\.1\\n    digest' engine/crates/shell/tests/agents.rs   # 1
grep -rnE '/(Users|home)/|/(private/)?tmp/' packages/agent-codex packages/agent-claude-code --exclude-dir=target --exclude-dir=dist | wc -l   # 0
```

The vector changes, against decision 7's declaration and then as the local gate reads them (with
the interpreter that has PyYAML):

```bash
/usr/bin/python3 tools/vector_delta.py "$BASE" WORKTREE --declared docs/work-orders/203-declared.yaml; echo "exit $?"
/usr/bin/python3 tools/vector_delta.py "$BASE" WORKTREE --declared-changed | tail -n 2; echo "exit ${PIPESTATUS[0]}"
```

The first prints one `fixture` line for each path of each shape of the declaration, each
`"declared": true` (eight for `registration-live-agents`); then
`Declarations read from docs/work-orders/203-declared.yaml: S shapes.`, with S the shapes,
`Vector delta: C changes; 0 flips; 0 undeclared; 0 errors.`, with C the lines before, and
`exit 0` (`1 shapes` and `8 changes` where the start checks listed that case alone). The second reads the
new file as untracked and prints the same two lines and `exit 0`. Report each last line; if any
other change appears, stop as decision 7 says.

The contracts and manifests, read as text and as YAML (with the interpreter that has PyYAML):

```bash
/usr/bin/python3 - "$BASE" <<'EOF'
import copy, re, subprocess, sys, yaml
base = sys.argv[1]
def show(ref, path):
    return subprocess.run(["git", "show", f"{ref}:{path}"], capture_output=True, text=True, check=True).stdout
order = show("origin/main", "docs/work-orders/203-new-conversation.md")
block = order.split("```yaml\n", 1)[1].split("```", 1)[0]
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
        item["tools"]["#new-conversation.v1"] = "new-conversation"
    print(name, "manifest", m["version"], "equal", int(m == want), len(held))
EOF
```

It prints, for each package, `NAME 0.2.0 1 0.1.1 1 operations 1 entries 2 same item 1` and
`NAME manifest 0.2.1 equal 1 K`, `K` not `0` (the items that implement `turn.v1`). The second
line applies decision 2 to `BASE`'s manifest, read as YAML, and finds the new manifest equal to
the result, so no other item and no other member changed. The first line is 050a's `.v1` check as
NAM-024 bounds it: with exactly decision 1's additions removed, the file is 0.2.0's byte for
byte, and 0.1.1's is its byte prefix. It reads decision 1's Operation from this order's text on
`main`, which governs where your tree's copy is older.

In each of `packages/agent-codex/` and `packages/agent-claude-code/`, one command at a time:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release
cargo run --release --bin package
```

`cargo test --release` passes every test with 0 failed, the new tests and the renamed one among
them, and every test target other than `tests/wrapper.rs` reports the totals it reported at
`BASE`. Report the totals, what `new_conversation_shares_no_context_without_restart` printed,
taken from
`cargo test --release --test wrapper new_conversation_shares_no_context -- --nocapture`, and the
digest that `package` prints: it is a host build's. Say on which system the tests ran.

Then show that the new test fails when a new conversation keeps Codex's thread. In
`packages/agent-codex/`, run `cp src/lib.rs target/lib.rs.fixed`; make one edit to `src/lib.rs`
so that a new conversation keeps the thread it should forget, and report the edit; run
`cargo test --release --test wrapper new_conversation_shares_no_context`, which must fail, in its
`v1` case, at step 4's assertion that the turn's `args` hold no `resume`. Restore with
`cp target/lib.rs.fixed src/lib.rs` (not `mv`, so that cargo rebuilds), rerun the same command,
which passes, and check that `cmp target/lib.rs.fixed src/lib.rs` prints nothing.

Then run the engine's checks of decision 7's copies. In `engine/`, one command at a time:

```bash
cargo clippy --all-targets -- -D warnings
cargo test --release --test agents
cargo run --release -p conduit-conformance --example make_archives -- --check
```

Do not run the engine's full `cargo test --release`: `tools/gate.py` runs it once before the merge
(ruling 80; ruling 89, point 2), and the verifier may run it once. The target `agents` passes with
0 failed, and its totals are those at `BASE`: decision 7 adds no test. The last command's last
line is the one the start checks kept, `N reproducible archive fixtures verified` with the same N:
the release `agents-0.2.0`, the synthetic TPL-005 package and every other generated byte are as at
`BASE`, but for the two copies' archives. Then, one command at a time in `engine/`, for
`registration-live-agents`, each case the start checks listed and the TPL-005 case,
`cargo run --release -p conduit-conformance -- .. CASE_ID`: each prints a line beginning
`PASS CASE_ID` and `1 cases passed` (more where another case's ID begins with `CASE_ID`), and no
`FAIL`. Where 050n wrote the TPL-005 case as a shell test, run that test alone by name.
`live_s1_scripted_agents_call_engine_with_distinct_run_identities` passes, so the copies equal the
0.2.1 package files and the pinned version and digests match. Report each command's last line
and the totals.

Then check the contracts with the engine, which the package tests cannot: that the loader accepts
the new Operation and a Conduit that lists it, that TPL-004 finds each Type's default, and that
one store admits 0.2.1 beside 0.2.0. Change nothing under `engine/` for it. Run each `conduit`
command below under `env -i PATH=/usr/bin:/bin HOME=T/home`, with T the temporary directory
below, so that no real client is on the `PATH` and no real home is reachable (`env -i` with a
command prints no environment, so the rule above allows it).

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
  must not be accepted.
- Then, in a second store, T/both, for each package: register a project of one `.v1` Agent
  template that pins version 0.2.0 with the archive and digest you kept before any change, then
  another project, its template under a name of its own, that pins 0.2.1 with the new archive.
  Both must be accepted with no diagnostic: this shows that one store admits 0.2.1 beside 0.2.0
  with no default changed, since the engine compares only the two releases' defaults there; the
  evidence for NAM-024's bound is the Python check above. If the second is refused, report its
  diagnostic and how the two projects were written, and go on: the coordinator judges it.

Report each reply's `accepted` and its diagnostics, and that no `register` reply names a started
process. If a `.v2` template is refused for an input
that order 050n made required, report the diagnostic and stop: do not change the template beyond
the rule above.

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark
each choice you made yourself as your own, and give the last line of every check and each count
above. Where the sandbox denies PID-scoped `ps`, the `.v2` stops read no process table, so step 3
of the new test finds `stopConfirmed` false; say so, report that test as failed for that reason,
and the reader reruns it on the Mac.

The admin then runs `python3 tools/check_private_paths.py` and `git diff --cached --check` on the
staged change. The order runs one Codex round, a worker and a fresh Codex verifier
([ruling 87](../proposals/ruling-87/README.md), point 1). A fresh read-only Opus reader reads the
result before it merges ([ruling 79](../proposals/ruling-79/README.md), point 2); where the
verifier reports a must, the reader confirms or refutes each such finding, and the Opus fixer
applies the confirmed ones with the reader's own (ruling 87, point 2). The local gate,
`tools/gate.py`, runs before the merge ([ruling 80](../proposals/ruling-80/README.md)), and a
read with no must merges on its pass ([ruling 85](../proposals/ruling-85/README.md), point 1),
and, on either base, only after the S1 rerun's pin (ruling 89, point 7), so that the build the
rerun measures holds no 0.2.1 (decision 3); its
row `vector changes` reads `203-declared.yaml` through `--declared-changed`, since this build's
pull request adds it. This order is not on the sign-in chain. Dispatched before 050n merges, it
falls in A77's pause ([ruling 89](../proposals/ruling-89/README.md), point 6): it is one of A77's
orders, not paired, and the coordinator's A77 record says why; dispatched after, it is paired or
not as A77's registration has it, and the coordinator says which before dispatch.

Where the coordinator dispatched this order on 050n's round-1 branch, the result is read there,
with `tools/gate.py --base` set to `BASE`, since `origin/main` lacks 050n's work. It merges only
after 050n has merged, and then only once the coordinator has done the following (the
coordinator's, under the lever of ruling 89, point 2):

- rebase this order's branch onto `main` with `git rebase --onto origin/main BASE`, so that only
  this order's commits are replayed onto 050n's squash, committing and rebasing under the work
  orders' identity; where 050n's branch was rebased or fixed after this order started, `BASE` is
  still the commit to rebase from;
- resolve any conflict where 050n's reader or fixer changed, after its round 1, what this order
  changes (each package's `conduit.yaml`, `contracts.yaml`, `Cargo.toml`, `Cargo.lock`,
  `src/lib.rs`, `src/main.rs`, `tests/wrapper.rs` and `README.md`, the copies under
  `conformance/fixtures/agents/`, the pins of decision 7, `make_archives.rs` and the cases' pin
  files), keeping 050n's merged text and applying this order's decisions to it; never resolve an
  archive or a digest by hand: copy the packages' files to the copies again, run `make_archives`
  in write mode, set the pins to the new digests, write `engine/STANDING.md` with
  `python3 tools/promise_standing.py --write`, and redo the declaration where its paths changed;
- run this order's start check in a clean worktree of `main` after 050n's merge, and where 050n's
  merge changed anything a decision reads (the tools a wrapper lists, a test that asserts them,
  the cases that name the copies, the TPL-005 case, the archives' listing), amend this order's
  text and the build to it, saying so in the Record;
- run again each package's `cargo test --release`, the engine's target `agents`, `make_archives`'
  check, the single-case runs, the contract check and `vector_delta.py` with this order's
  declaration, then `tools/gate.py` against `origin/main`; and
- have the reader read the rebase's resolutions and any amendment as new content (ruling 79,
  point 2), and write this order's Record.
