# Work order 050e (P2e): failed turns as fields, with a network class, and a lapsed login in a turn (the full review's order 17)

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

## What and why

**Where this order starts.** Your tree is the branch that the coordinator names as `BASE`, not
`main`: `wo/050-agents-0.2.0`, which holds orders 050a, 076 and 078, 050a's Record, and orders 050c
and 050d, each with its Record, rebased onto `main`. `BASE`'s tip holds this order file, committed:
the coordinator rebases the branch onto `main` after this order's text merges, and sets `BASE` to
the branch's tip. Before you change anything, check from the repository root that you find there
what that gives, and stop, change nothing and report if any check prints otherwise:

```bash
git merge-base --is-ancestor f414e438 "$BASE"; echo "main $?"
test "$(git rev-parse HEAD)" = "$(git rev-parse "$BASE")"; echo "head $?"
git ls-files --error-unmatch docs/work-orders/050e-agents-v2-failed-turn-fields.md >/dev/null; echo "order $?"
git status --short | wc -l
grep -c '^## Record' docs/work-orders/050a-agents-v2-own-tools.md docs/work-orders/050c-agents-v2-client-home.md docs/work-orders/050d-agents-v2-process-tree.md
test "$(git rev-parse "$BASE:packages/agent-codex")" = 6f15b766bf779119a396b40eb5adde3d197f00fa; echo "codex $?"
test "$(git rev-parse "$BASE:packages/agent-claude-code")" = d101a6b6a97013b7632cd15cf2c6964008160175; echo "claude $?"
grep -cE 'FailedTurn|StopReport|failedTurn' packages/agent-codex/src/lib.rs packages/agent-codex/src/main.rs packages/agent-claude-code/src/lib.rs packages/agent-claude-code/src/main.rs
diff packages/agent-codex/src/lib.rs packages/agent-claude-code/src/lib.rs | grep -c '^[<>]'
diff packages/agent-codex/src/main.rs packages/agent-claude-code/src/main.rs | grep -c '^[<>]'
grep -cE '#\[(tokio::)?test' packages/agent-codex/tests/wrapper.rs packages/agent-claude-code/tests/wrapper.rs
```

They print `main 0`, `head 0`, `order 0` and `0`; then `1` for each order file (the three
Records); `codex 0` and `claude 0` (the two packages are those 050d's Record describes, since no
commit on `main` after the cut touches them); `0` for each of the four sources; `2` (the two
`lib.rs` differ in line 18 alone, the `CLAUDE` constant); `0` (the two `main.rs` are one file); and
`14` for each `wrapper.rs`. The line numbers below are those of `agent-codex` at that tree, and
hold for `agent-claude-code`'s `lib.rs` and `main.rs` too; its `tests/wrapper.rs` has its own.

**What it builds.** A failed `.v2` turn is today one text: `client turn K failed: CAUSE; session
cannot restart implicitly`, with order 050d's survivor note appended, and its client failure's
class has no class for a lost network, so a network loss reads `other`. This order gives the failed
turn its fields, in both packages: a `network` class in the one class table, and, for `.v2`, the
failed call's result carries the turn's number, its cause, its class, whether the client had
started, the PIDs that outlived the stop and whether the stop was confirmed, as one JSON object in a
second text block of its `content`, holding fixed words and numbers alone. The survivors of 050d's
stop move from the text into those fields; on TERM they stay on the wrapper's stderr. A login that
lapses in a turn fails it with the class `authentication`, and the turn sends no URL request,
which is the half of F84 that a plain item follows.

It carries [the full review's order 17](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/full-review/orders.md):
[FR-338](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/full-review/findings/P06-dogfooding.md#fr-338) as its verifier corrected it,
that is, a `network` class by the same fixed-substring method and "the failed result carries its
class, the turn's number and whether the client had started as fields, not only as text", with no
client text kept (the verifier's point 3: a kept tail of client error text would reverse 050a's
rule against client text in results). Its done-when: "in each package, synthetic failures map to
each class, `network` included; 050a's byte-for-byte `.v1` check passes; Rust checks". FR-338's
other proposals are not here: the un-refusing of a failed conversation is struck for `.v2`
([ruling 82](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-82/README.md#the-decisions), point 5), the kept tail of client
text is the verifier's correction, and raising Codex's own retry limits waits on FR-338's probe
P1, which has not run.

From the targeted review of the 0.2.0 sign-in surface it carries item 7 of both
[`claude-code.v2`](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/review-signin-2026-11-16/README.md#050-claude-codev2) and
[`codex.v2`](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/review-signin-2026-11-16/README.md#050-codexv2) ("As `claude-code.v2`'s
item 7"): "`turn.v1`'s schemas stay byte for byte as 0.1.1 published them; #17's class and turn are
not outputs and no schema or description enumerates the classes; the survivors of order 16's stop
are named where #17's fields live and, on TERM, in the wrapper's own log; the turn implementation
follows the half of F84 that matches its item. Done when 050a's byte check passes and a vector shows
the fields reach the caller under IFC-025 wherever 050 places them." Its verdicts M65 to M67
([verdicts.md](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/review-signin-2026-11-16/verdicts.md)) and its findings
[V18](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/review-signin-2026-11-16/findings.md#v18-where-order-17s-fields-and-order-16s-survivors-live)
and [V28](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/review-signin-2026-11-16/findings.md#v28-a-login-that-lapses-during-a-turn)
say why. Order [050d](050d-agents-v2-process-tree.md) named the survivors in the failed turn's text
until this order and on TERM on the wrapper's stderr, the coordinator's reading of "the wrapper's
own log"; this order moves the first into the fields and leaves the second. The vector of item 7's
done-when is order 050n's, which owns `conformance/` and moves the Agent fixtures to 0.2.0 (the
coordinator's plan of 050, in [order 117](117-plan-s2-s3.md)'s 050 entry); this order adds none.

**The turn item's kind, and so where the fields live.** The review left open whether the item that
implements `turn.v1` carries Conduit's keys, and noted that #17's carrier depends on it (G07, in
its section 1's line [o1](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/review-signin-2026-11-16/README.md#1-the-open-line-clause-by-clause));
the owner's ruling 82, point 6, keeps it 050's choice. The coordinator chooses here that the turn
items stay plain, as `conduit.yaml` gives them at `BASE` (`wrapper`, `wrapper-v2` and, in
`agent-codex`, `wrapper-v2-path`, each `plain: true`; verdict M18 freezes `wrapper-v2` so), and so
the carrier is the failed result's `content`, stated as such:

- [IFC-025](../../spec/interface/implementation.yaml) bounds where the fields can live. A result
  the engine gives a consumer whose `isError` is true "carries none of the operation's outputs,
  `diagnostics` apart", and "Where its outcome came from a result of an implementation marked
  `plain`, its `content` holds that result's `content`, unchanged, after that block where there is
  one"; and of an error result and `outputSchema`, "This specification takes it that it need not"
  conform. So a plain wrapper's second text block reaches the caller unchanged, and `turn.v1`'s
  `outputSchema`, closed and shared with `.v1`, is not touched: the fields are not outputs.
- A keyed turn item would end a lapsed login in the form the engine reads (F84's keyed half, which
  [order 048a-2](048a2-build-sign-in-lapse-and-conflict.md)'s decision 1 writes into PND-005: "an
  implementation that carries Conduit's keys ends the call failed or unknown, as it can tell, with
  an error diagnostic naming PND-005", and its decision 5 has the engine raise the instance's
  sign-in action from that diagnostic). But IFC-025 and IFC-009 promise no keyed carrier for these
  fields as data: no `_meta` key beyond those IFC-025 names; the engine as 048a-2 reads it passes
  none of a keyed implementation's `structuredContent` outside succeeded (048a-2's reading of
  `complete`: it "passes on none of the implementation's `structuredContent` for an outcome other
  than succeeded"); IFC-025 passes an implementation's `content` only where it is `plain`; and a
  diagnostic's message is passed only in the engine's wording (IFC-050: "Wording is the engine's
  choice"). So the keyed alternative needs a change to `spec/` that names a carrier the engine
  passes on for these fields as data, which item 7 does not carry. A keyed item is also a Conduit-aware implementation (IFC-006's keys, IFC-009's outcome, IFC-026's
  filters), well beyond one small order.
- The cost, stated in the README as a limit: under F84's plain half, "a plain one reports it only
  by a URL request, under IFC-016", and a turn has no login URL to give, so it sends none. A login
  that lapses during a turn then fails the turn with the class `authentication`, the engine ends the
  call unknown under IFC-010, and no sign-in action is raised from the turn (V28's scenario, less
  its silence: the caller reads the class). The lapse is found at the instance's next account check
  (PND-005's first two triggers, PND-011). So for a lapse found in a turn,
  [IN_FORCE.md](../../IN_FORCE.md)'s "A lapsed login raises PND-005, and model work reports
  unavailable naming that action" waits for that check, a tension V28 and
  [order 171b](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders/171b-sign-in-return-and-holds.md) record as standing; the keyed alternative would
  meet it. The coordinator puts this choice on
  [the owner's review list](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/first-build/to-review-with-the-owner.md) with this order's
  Record, beside the keyed alternative and its `spec/` change.

The README cites PND-005's two forms as `spec/` will hold them once 048a-2's work merges, which is
before 048b-join and so before 050n carries this branch to `main`.

**What the branch already does, so that you do not redo it.** In each package's `src/lib.rs` (the
two files are one source; line 18's `CLAUDE` selects the client) and `src/main.rs` (one file):

- `ERROR_CLASSES` (`lib.rs:477-505`) holds `usage-limit`, `rate-limit`, `authentication` and
  `context-window`, each with fixed substrings, and `classify` (`:506-512`) gives the first class
  whose substring the lowercased text holds, or `other`. Both versions use it (050a's decision 7).
- `ClientTree::stop` (`:730-779`) returns `Option<String>`: `None`, a `TreeReading`'s `note()`
  (`:664-674`: `survivor_note` of its PIDs, then `TREE_UNCONFIRMED` after `; ` where the reading is
  unconfirmed), or, through `unconfirmed_stop` (`:839-849`), `Some(TREE_UNCONFIRMED.into())`. Two
  paths park for good once TERM's stop has begun. `survivor_note` (`:870-881`) and
  `append_stop_note` (`:883-885`) build the note.
- `Session` (`:941-948`) keeps `stream`, `failed` and `turns_started`. `Session::turn`
  (`:963-989`) records a failure's cause in `failed` and returns `end_failed_turn(&cause)`; later
  turns are refused with `session failed at turn K: CAUSE; implicit restart refused` before any
  launch. `Session::cancel` (`:1100-1103`) does the same with `cancelled`. `end_failed_turn`
  (`:1104-1114`) runs `shutdown` and returns `anyhow::Error::msg` of
  `client turn K failed: CAUSE; session cannot restart implicitly`, followed by `; ` and the note
  where `shutdown` returned one. `Session::shutdown` (`:1115-1131`) stops the tree for `.v2`,
  kills and reaps the client and sets `stream` to `None`.
- The causes are fixed texts: `deadline: ...`, `cancelled`, `client failure: CLASS`,
  `client exit: ...`, `EOF before turn boundary`, `no thread`, `invalid client output`,
  `resume changed thread`, `client did not start`, and, for `.v2`, the home check's refusals, which
  begin `client home (` (order 050c's `client_home`, `:186-240`).
- `main.rs`'s `call_tool` (`:30-71`) answers a failed turn with
  `{"content":[{"type":"text","text":TEXT}],"isError":true}` (`:64-66`) and no
  `structuredContent`. The TERM task (`:104-117`) writes `{WRAPPER}: {note}` on stderr where the
  stop returns a note, and the end of input (`:124-135`) writes the same line.
- The scripted client's `FAIL:TEXT` ends a turn with `turn.failed` (Codex) or a `result` with
  `is_error` true (Claude Code) whose message is `sentinel-secret TEXT`.

## The decisions it applies

1. **The `network` class.** In `ERROR_CLASSES`, add one entry after `context-window`, so that the
   classes stay in this order and `other` stays the fallback:
   `("network", &["network", "connection", "disconnected", "error sending request", "timed out", "timeout", "dns", "getaddrinfo", "econnreset", "econnrefused", "enotfound", "unreachable", "fetch failed"])`.
   `classify` does not change. Both versions use the one table, as they use its other classes. Like
   the others, these substrings are a first list, not checked against real failures.

2. **What a stop leaves, as a value.** Add, after `survivor_note`:

   ```rust
   /// What a .v2 stop left: the PIDs that outlived it, ascending, and whether it is unconfirmed.
   #[derive(Clone, Debug, Default, PartialEq, Eq)]
   pub struct StopReport {
       pub survivors: Vec<i32>,
       pub unconfirmed: bool,
   }
   ```

   with `pub fn note(&self) -> Option<String>`, which gives what `TreeReading::note` gives:
   `survivor_note(&self.survivors)`, then `TREE_UNCONFIRMED` after `; ` (or alone) where
   `unconfirmed`. `ClientTree::stop` returns `StopReport`: each path that returned
   `reading.note()` returns the reading's PIDs, sorted ascending, and its `unconfirmed`; each path
   that returned `Some(TREE_UNCONFIRMED.into())`, `unconfirmed_stop` included, returns no PID with
   `unconfirmed: true`; and the path that returned `None` returns `StopReport::default()`. The two
   parking paths do not change. So on every path `stop(..).await.note()` is byte for byte what
   `stop` returned at `BASE`. `TreeReading::note` goes, or becomes that conversion.
   `Session::shutdown` returns `StopReport` (the default for `.v1`, which stops no tree). In
   `main.rs` the TERM task and the end of input call `.note()` on what they get, and write the same
   line as at `BASE`.

3. **The failed turn.** Add, after `StopReport`:

   ```rust
   /// A failed turn (the full review's order 17): fixed words and numbers, never client text.
   #[derive(Clone, Debug, PartialEq, Eq)]
   pub struct FailedTurn {
       pub turn: usize,
       pub cause: String,
       pub client_started: bool,
       pub stop: StopReport,
       v2: bool,
   }
   ```

   - `Display` writes `client turn {turn} failed: {cause}; session cannot restart implicitly`, and
     `FailedTurn` implements `std::error::Error`.
   - `pub fn kind(&self) -> &'static str` gives the cause's kind by its fixed text: `deadline` for
     a cause beginning `deadline: `; `cancelled`; `client-failure` for one beginning
     `client failure: `; `client-exit` for one beginning `client exit: `; `eof` for
     `EOF before turn boundary`; `no-thread`; `invalid-client-output`; `resume-changed-thread`;
     `client-did-not-start`; `client-home` for one beginning `client home (`; and `unclassified`
     for any other.
   - `pub fn class(&self) -> Option<&str>` gives the cause after `client failure: `, or `None`.
   - `pub fn fields(&self) -> Option<Value>` gives `None` for `.v1`, and for `.v2`
     `{"failedTurn":{"turn":K,"cause":KIND,"class":CLASS,"clientStarted":B,"survivors":[P1,P2],"stopConfirmed":B}}`,
     with `class` null where `class()` is `None`, `survivors` the stop's PIDs (an empty list where
     none) and `stopConfirmed` the negation of the stop's `unconfirmed`.

   `end_failed_turn` reads `client_started` as `self.stream.is_some()` before it runs `shutdown`
   (true where the turn launched its client or found one running, as a Claude Code conversation's
   later turns do; false where the launch or the home check failed), then returns
   `anyhow::Error::new(FailedTurn { .. })` with `turn` `self.turns_started`, the cause, the
   shutdown's `StopReport` and `v2` from the config. The text no longer carries the note: for
   `.v2` the survivors live in the fields alone, and for `.v1` the text is as at `BASE`, since a
   `.v1` stop has no note. `failed` keeps the cause alone, so the refusal of later turns does not
   change, and it carries no fields. `Session::turn` and `Session::cancel` keep their signatures;
   `append_stop_note` stays only where `StopReport::note` uses it.

4. **The failed result.** In `main.rs`, import `FailedTurn`, and make the failed arm:

   ```rust
   Err(error) => {
       let mut content = vec![json!({"type":"text","text":error.to_string()})];
       if let Some(fields) = error.downcast_ref::<FailedTurn>().and_then(FailedTurn::fields) {
           content.push(json!({"type":"text","text":fields.to_string()}));
       }
       json!({"content":content,"isError":true})
   }
   ```

   The result keeps no `structuredContent` and no `_meta`, and its `isError` stays true. A `.v1`
   failure, the refusal of a later turn and an invalid configuration's answer keep one block. The
   two `main.rs` stay one file.

5. **A lapsed login in a turn.** No code beyond decisions 1 to 4: the items stay plain, the turn
   sends no URL request and no `elicitation`, and a lapse fails the turn with its class. Decision
   6's test checks it.

6. **The tests**, in each package's `tests/wrapper.rs`, the same in both:
   - **`fixed_failure_causes_in_both_versions`** gains, in both versions, the cases
     `("FAIL:stream disconnected before completion", "client failure: network", false)`,
     `("FAIL:Connection error.", "client failure: network", false)`,
     `("FAIL:Request timed out", "client failure: network", false)`,
     `("FAIL:error sending request for url", "client failure: network", false)` and
     `("FAIL:401 connection reset", "client failure: authentication", false)`, the last showing
     that a class listed earlier wins. `assert_failure` keeps its checks on the texts and adds,
     on the failed turn's error: it downcasts to `FailedTurn`, with `turn` K and `cause` the
     case's; for `.v2`, `fields()` equals the object of decision 3, with `cause` the kind that the
     test reads from the case's cause by the fixed texts of decision 3 (written in the test, not
     by calling `kind`), `class` the case's class or null, `clientStarted` false for
     `client did not start` alone, `survivors` an empty list and `stopConfirmed` true; for `.v1`,
     `fields()` is `None`. The refusal's error does not downcast to `FailedTurn`, and the fields'
     text holds no `sentinel-secret`.
   - **`v2_client_tree_stop_leaves_no_survivor`**: for the deadline and cancellation cases the
     text now equals the expected text exactly; the error downcasts to `FailedTurn`;
     `PlantedTree::check_report` runs, as at `BASE`, on the expected text followed by `; ` and
     `stop.note()` where that is `Some`, so each live planted PID is named in the fields; and the
     fields' `survivors` equal `stop.survivors` and `stopConfirmed` the negation of
     `stop.unconfirmed`. The `.v1` check also asserts that `fields()` is `None`. For each `.v2`
     case the test `eprintln!`s the fields' JSON beside its planted PIDs. The TERM case does not
     change: its stderr line is the wrapper's own log.
   - **A new test, `v2_failed_turn_fields_reach_the_result`**, through the wrapper binary over
     stdio, started as `stdio_v2_defaults_and_invalid_configuration_remain_callable` starts it,
     with `AGENT_ACCOUNT_SERVICES` `false`, and, for `.v2`, `AGENT_TURN_DEADLINE_SECONDS` `60` and
     `CONDUIT_STATE_DIR` the config's; for `.v1`, `AGENT_PEER_MESSAGING` `false` and neither of
     those two. Each case reads the call's result as JSON (`serde_json::to_value`):
     - `.v2`, `FAIL:stream disconnected before completion`: `isError` true; `content` holds two
       text blocks, the first
       `client turn 1 failed: client failure: network; session cannot restart implicitly` and the
       second, parsed as JSON,
       `{"failedTurn":{"turn":1,"cause":"client-failure","class":"network","clientStarted":true,"survivors":[],"stopConfirmed":true}}`;
       no `structuredContent` and no `_meta` member. A next `TEST` call answers `isError` true with one block,
       `session failed at turn 1: client failure: network; implicit restart refused`.
     - `.v2`, in a fresh wrapper, `FAIL:401 unauthorized`, the lapse: as above, the next `TEST`
       refusal included, with class `authentication`; and the result has no `_meta` member, no
       `resultType` but `complete` where it has one, and nowhere the text `elicitation`.
     - `.v1`, `FAIL:stream disconnected before completion`: one block, its text
       `client turn 1 failed: client failure: network; session cannot restart implicitly`.
     - In every case, neither block nor the wrapper's stderr holds `sentinel-secret`,
       `synthetic-test-token`, `synthetic-anthropic-key` or `synthetic-openai-key`.
     - In every case the test `eprintln!`s both blocks of the result (one where it has one).

   The test count goes from 14 to 15 in each package. Every other test keeps passing unchanged.

7. **The READMEs**, both, in the package's own client's terms. Write no absolute local path and no
   client version.
   - In `## The .v2 client's process tree`, "names each PID that outlived it in the failure text.
     On TERM the wrapper writes that note to its stderr" becomes "names each PID that outlived it
     in the failed call's fields. On TERM the wrapper writes the note
     `client tree: PIDs P1, P2 outlived the stop`, or
     `client tree: the process table could not be read, so the stop is unconfirmed`, to its
     stderr", with
     "fields" a link to the new section's anchor, `the-v2-failed-turns-fields`.
   - In `## Turn boundaries and failures (both versions)`, the three lines that begin "A `.v2` call
     adds" become "A `.v2` failed call also holds the failed turn's fields.", with "failed turn's
     fields" a link to the same anchor.
     The class table gains the row `network` with decision 1's substrings, before `other`.
   - Add, at the end, `## The .v2 failed turn's fields`, which says: a `.v2` failed call's
     `content` holds, after its text, a second text block holding one JSON object,
     `{"failedTurn":{...}}`, with `turn`, `cause`, `class`, `clientStarted`, `survivors` and
     `stopConfirmed` as decision 3 gives them, and a table of each `cause` beside the cause text it
     comes from; the fields hold fixed words and numbers alone, never client text; they are not
     outputs of `turn.v1`, whose schemas are 0.1.1's, the failed result has no
     `structuredContent`, and the engine gives a plain implementation's `content` to the caller
     unchanged after its own text block (IFC-025); the classes are listed in this README and in no
     schema or description; a `.v1` failed call and the refusal of a later call hold their text
     alone. And, as a limit: the turn items are plain, so under PND-005 they report a missing or
     lapsed login only by a URL request (IFC-016); a turn has no login URL to give and sends none,
     so a login that lapses during a turn fails it with the class `authentication` where the
     client's text matches, the engine ends the call unknown under IFC-010 and raises no sign-in
     action for it, and the lapse is found at the instance's next account check.

8. **What this order does not do.** No change to `conduit.yaml`, `contracts.yaml`, `Cargo.toml`,
   `Cargo.lock`, `tests/fixtures/client.rs`, `src/bin/package.rs` or the other tests, and none
   under `engine/`, `conformance/`, `spec/` or `docs/`. No item's kind changes, no description
   changes (item 6 of the review's lists holds each description to exactly the sentences of Q1's
   answer, which order 050f and later orders write), no output is added to `turn.v1`, no client
   text is kept, no un-refusing of a failed conversation, no fields on the refusal of a later
   turn, and no change to Codex's retry settings. The IFC-025 vector is order 050n's. The engine's test
   `live_s1_scripted_agents_call_engine_with_distinct_run_identities`
   (`engine/crates/shell/tests/agents.rs`) fails on this branch by design until order 050n moves
   `conformance/fixtures/agents/` to 0.2.0, as [order 050a](050a-agents-v2-own-tools.md) records.
   It is the one expected failure on `wo/050-agents-0.2.0`, and it is not this order's: neither you
   nor the verifier counts it against this order. Do not run the engine's tests and do not change
   it.

## Who decided what

The carrier is the full review's order 17, under D55, which stays the coordinator's, open to the
owner's strike ([ruling 82](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-82/README.md#what-it-amends), the section after
point 6: "The rest of D55 and D80 stays the coordinator's"); ruling 82 point 5 struck only D55's
un-refusing for `.v2`. FR-338 as its verifier corrected it is the review's. Item 7 of the review's
must-state lists, which the owner accepted with ruling 82, carries M65 to M67; ruling 82 point 6
(o1) leaves the turn item's kind to 050. The split of order 050 into 050b to 050o, this order's
place on `wo/050-agents-0.2.0` after 050d and 048a-2's text, and its base are the coordinator's,
open to the owner's strike ([ruling 48](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-48/README.md)). Everything else here is
the coordinator's too, each open to the owner's strike: the turn items kept plain (G07), with the
cost stated in the README's limit; the fields as a second text block of the failed result's
`content`; their names, the cause kinds and `unclassified`; `clientStarted` read from whether the
session holds a client; the survivors moved out of the text into the fields; the refusal of a
later turn left without fields; the `network` class in both versions, its place after
`context-window` and its substrings; the description left unchanged, so that the fields are named
in the README alone; and the README's wording. Apply them as written and report them under
`decisions` as the coordinator's.

## Scope

Change only, in `packages/agent-codex/` and `packages/agent-claude-code/`: `src/lib.rs`,
`src/main.rs`, `tests/wrapper.rs` and `README.md`. Nothing else, in the packages or outside them.
The build's ignored output (`target/`, `dist/`) may change.

## Done when

From the repository root, each passes:

```bash
python3 tools/check_docs.py
python3 tools/check_private_paths.py
git diff --check
```

Each prints what is said beside it:

```bash
git status --short | grep -vE '^ M packages/agent-(codex|claude-code)/(src/lib\.rs|src/main\.rs|tests/wrapper\.rs|README\.md)$' | wc -l   # 0
git diff --quiet "$BASE" -- packages/agent-codex/conduit.yaml packages/agent-claude-code/conduit.yaml packages/agent-codex/contracts.yaml packages/agent-claude-code/contracts.yaml packages/agent-codex/Cargo.toml packages/agent-claude-code/Cargo.toml packages/agent-codex/Cargo.lock packages/agent-claude-code/Cargo.lock packages/agent-codex/tests/fixtures packages/agent-claude-code/tests/fixtures; echo $?   # 0
for NAME in agent-codex agent-claude-code; do head -c "$(wc -c < conformance/fixtures/agents/$NAME/contracts.yaml)" packages/$NAME/contracts.yaml | cmp - conformance/fixtures/agents/$NAME/contracts.yaml; echo "$NAME $?"; done   # agent-codex 0, agent-claude-code 0
grep -cE 'usage-limit|rate-limit|authentication|context-window|client-failure|failedTurn|clientStarted|stopConfirmed' packages/agent-codex/contracts.yaml packages/agent-claude-code/contracts.yaml   # 0 each
diff packages/agent-codex/src/lib.rs packages/agent-claude-code/src/lib.rs | grep -c '^[<>]'   # 2
diff packages/agent-codex/src/main.rs packages/agent-claude-code/src/main.rs | grep -c '^[<>]'   # 0
grep -c 'pub struct FailedTurn' packages/agent-codex/src/lib.rs packages/agent-claude-code/src/lib.rs   # 1 each
grep -c 'pub struct StopReport' packages/agent-codex/src/lib.rs packages/agent-claude-code/src/lib.rs   # 1 each
grep -c 'the-v2-failed-turns-fields' packages/agent-codex/README.md packages/agent-claude-code/README.md   # at least 2 each
grep -c '^## The .v2 failed turn.s fields$' packages/agent-codex/README.md packages/agent-claude-code/README.md   # 1 each
grep -c '^| `network` |' packages/agent-codex/README.md packages/agent-claude-code/README.md   # 1 each
grep -c 'A `.v2` call adds' packages/agent-codex/README.md packages/agent-claude-code/README.md   # 0 each
grep -cE '#\[(tokio::)?test' packages/agent-codex/tests/wrapper.rs packages/agent-claude-code/tests/wrapper.rs   # 15 each
grep -rnE '/(Users|home)/|/(private/)?tmp/' packages/agent-codex packages/agent-claude-code --exclude-dir=target --exclude-dir=dist | wc -l   # 0
```

The third line is 050a's byte check: the 0.1.1 contracts, which hold `turn.v1`'s schemas, stay a
byte prefix of each package's `contracts.yaml`.

In each of `packages/agent-codex/` and `packages/agent-claude-code/`, one command at a time:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release
cargo run --release --bin package
```

`cargo test --release` reports, for `tests/wrapper.rs`, 15 passed and 0 failed, and every other
test target as at `BASE` (the library's unit test 1 and the package test 1 in each, and in
`agent-codex` the manifest test 1). Report the totals, the two blocks of each result that
`v2_failed_turn_fields_reach_the_result` saw, the planted PIDs and the fields that
`v2_client_tree_stop_leaves_no_survivor` printed, both taken from
`cargo test --release --test wrapper <name> -- --nocapture` for those two tests, and the digest
that `package` prints: it is a host build's. Say on which system the tests ran.

Then show that the new test fails without decision 4's second block. In `packages/agent-codex/`:

```bash
cp src/main.rs target/main.rs.fixed
```

Make one edit to `src/main.rs`: replace the statement
`content.push(json!({"type":"text","text":fields.to_string()}));` with `let _ = fields;`. Then run
`cargo test --release --test wrapper v2_failed_turn_fields`; it must fail, in its first case, at
the assertion that the `.v2` result holds two blocks. Restore with
`cp target/main.rs.fixed src/main.rs` (not `mv`, so that cargo rebuilds), rerun the same command,
which passes, and check that `cmp target/main.rs.fixed src/main.rs` prints nothing.

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark each
choice you made yourself as your own, and give the last line of every check and each count above.
Tests that bind loopback ports may fail in your sandbox; if they do, say so, and the admin reruns
them. Where the sandbox denies PID-scoped `ps`, `v2_client_tree_stop_leaves_no_survivor` stops at
its preflight; say so, report 14 passed and that one failed for that reason, and the reader reruns
it on the Mac.

The admin then runs `python3 tools/check_private_paths.py` and `git diff --cached --check` on the
staged change. This order is a checkpoint on `wo/050-agents-0.2.0`: it does not merge alone, and its
Record is written at the foot of this file on that branch, as orders 050c's and 050d's are. A fresh
Codex verifier and a fresh read-only Opus reader read the result before the checkpoint stands
([ruling 79](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-79/README.md), point 2). The local gate that
[ruling 80](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-80/README.md) has a merge wait on, `tools/gate.py`, runs at order
050n's merge, read with the expected engine failure known.

## Record

Run on 2026-11-18, from about 13:50 to 14:21 UTC, on the Mac from `BASE` `wo/050-agents-0.2.0` at
`7d1f01ed` (the coordinator's acceptance of the stop lock in order 050d's Record), in two rounds:
the worker (655 s) and a fresh Codex verifier (386 s), then the worker again (486 s) and a second
fresh Codex verifier (310 s). The change is the eight files of Scope, the four it names in each
package. `ERROR_CLASSES` gains `network` after `context-window`, in both versions;
`ClientTree::stop` and `Session::shutdown` return a `StopReport`, whose `note()` is byte for byte
the note of `BASE`; a failed turn's error is a `FailedTurn`, whose text no longer carries the stop
note and whose `fields()` give, for `.v2`, `{"failedTurn":{...}}` with `turn`, `cause`, `class`,
`clientStarted`, `survivors` and `stopConfirmed`; `main.rs` appends those fields as a second text
block of the failed result's `content`; each wrapper suite gains the five cases of
`fixed_failure_causes_in_both_versions` and the test `v2_failed_turn_fields_reach_the_result` (14
tests to 15); and both READMEs gain the `network` row and the section
`## The .v2 failed turn's fields`. The worker applied decisions 1 to 8 as written. Its own choices:
a private conversion that sorts the reading's PIDs and a private constant for an unconfirmed stop
with no survivor; a stdio caller that advertises both elicitation modes and records any request, so
that the test sees an elicitation as well as reading the JSON; and doc-tests run on their own after
the wrapper suite stopped. Its sandbox denied PID-scoped `ps`, so the tree test stopped at its
preflight in both packages (14 wrapper tests passed in each). Its mutation failed at the first `.v2`
result's block count, 1 against 2, and passed restored.

The first verifier, which ran no Rust suite in its read-only sandbox, found one must and three
shoulds: on MCP 2026-07-28 the error results omitted `resultType`, which IFC-001 and the protocol
require; the tree test could pass with survivors out of order, since the note sorts a copy and the
JSON assertion reused the report's vector; no passing case had `stopConfirmed: false`; and the
READMEs said every `.v2` error had two blocks, though an invalid configuration answers with one.
Round 2 built the failed result with `CallToolResult::error` and `ContentBlock::text`, which sets
`resultType` `complete` and which the MCP library strips for a legacy peer, and asserted it on
modern stdio and its absence on a legacy one, through a fourth case of the new test with the legacy
handshake; it compared the survivors with the independently observed live PIDs, ascending; it added
the library unit test `unconfirmed_stop_report_sorts_survivors`, which sends `[43, 41, 42]` with
`unconfirmed` through the report's conversion; it let `check_report` take the unconfirmed flag; and
it tied the README's second block to a failed turn. The second verifier found one must and four
shoulds: a successful turn on 2026-07-28 still omitted `resultType`, as at `BASE`; the read-error
fallback, `StopReport::UNCONFIRMED`, had no deterministic test; the failed-home cases did not check
the fields; the invalid-configuration case did not check the whole one-block shape; and the worker's
report lacked the totals of the other test targets.

A fresh read-only Opus reader then read the result (ruling 79, point 2), on a scratch copy on macOS
arm64 where PID-scoped `ps` is allowed. Every text check of "Done when" printed what the order says;
fmt and clippy were clean in both packages; `cargo test --release` gave 15 wrapper tests passed and
0 failed in each, the package test 1 in each and Codex's manifest test 1, but the library's unit
tests 2, not 1. `v2_client_tree_stop_leaves_no_survivor` passed (10.31 s for Codex) with survivors
`[]` and `stopConfirmed` true for the deadline and the cancellation and an empty TERM stderr, the
stdio blocks were the order's, and both digests equalled the worker's. The red check failed at the
block count, 1 against 2, and passed restored, with `cmp` silent. It found one must and five
shoulds:

- F1: a done-when is not met as worded: it asks for every other target as at `BASE`, the library's
  unit test 1, and each library now has 2.
- F2: the failed arm does not use decision 4's code as written, since round 2 builds the result with
  `CallToolResult::error` and `ContentBlock::text`, so the red check's statement is not in the file.
- F3: round 2 weakened 050d's live tree test: `check_report` took the unconfirmed flag from the very
  report it checks, and `stopConfirmed` was compared with that report's own flag, so an unconfirmed
  live stop passed, where decision 6 says `check_report` runs as at `BASE`.
- F4: the new README section never said the fields are not outputs of `turn.v1`, and said "Only this
  README lists the classes", which is false, as the sibling's README lists them too.
- F5: the README paraphrased decision 7's two replacement sentences.
- F6: on 2026-07-28 a successful turn still omits `resultType`, the second verifier's open must; it
  was so at `BASE`, and the order neither asks for nor forbids it, so it is the coordinator's call.

The fixer kept the branch on `wo/050-agents-0.2.0` at `7d1f01ed`, which had not moved, and applied
each finding, the same in both packages:

- F1: the test stays. The library's unit tests are 2 in each package;
  `unconfirmed_stop_report_sorts_survivors` was added by the worker's round 2 for the verifier's
  ordering and unconfirmed findings.
- F2: a departure from decision 4, as the worker's round 2's IFC-001 correction: the failed arm
  builds `CallToolResult::error` from `ContentBlock::text` blocks, which adds `resultType`
  `complete` on MCP 2026-07-28 and restructures the success arm. The wire result still has `isError`
  true, no `structuredContent` and no `_meta`. The red check's actual statement is
  `content.push(ContentBlock::text(fields.to_string()));`, and its observed failure was at
  `tests/wrapper.rs:1581`, the block count, 1 against 2 (at `tests/wrapper.rs:1593` after this
  round's lines).
- F3: the tree test calls `PlantedTree::check_report(&live, false, &report, &expected, "; ");` (the second reader's S4 later restored BASE's helper and TERM call) and
  asserts that the fields' `stopConfirmed` equals `true`, so an unconfirmed live stop fails the test
  again, as at `BASE`; the unconfirmed note stays covered by the library's unit test.
- F4: both READMEs read "The fields are not outputs of `turn.v1`, whose schemas stay as 0.1.1
  published them." and "The classes are listed in this README and in no schema or description."
- F5: both READMEs read "A `.v2` failed call also holds the failed turn's fields." and "failed
  call's fields. On TERM the wrapper writes the note `client tree: PIDs P1, P2 outlived the stop`,
  or `client tree: the process table could not be read, so the stop is unconfirmed`, to its stderr
  and exits with status 143.", with "failed turn's fields" and "fields" each a link to the anchor
  `the-v2-failed-turns-fields`.
- F6, which is also the second verifier's must, taken by the reader's second choice: a departure
  from decision 6, under which every other test keeps passing unchanged, and beyond decision 4,
  which shapes the failed arm alone: the success arm sets
  `result.result_type = Some(ResultType::COMPLETE)`, which the MCP library strips for a legacy peer.
  `stdio_v2_defaults_and_invalid_configuration_remain_callable` gains the case `modern`, a valid
  configuration served over the 2026-07-28 discovery, and asserts `resultType` `complete` on its
  successful calls and no `resultType` on every legacy reply, the successful ones included; the test
  count stays 15. Without the new line, the test failed at the modern success's `resultType`, `Null`
  against `complete`; restored with `cp`, it passed and `cmp` printed nothing. The two `main.rs`
  stay one file.

Of the second verifier's four shoulds, the totals are given below; the deterministic test of the
read-error fallback, the failed-home fields and the full invalid-configuration shape were not
applied, since the reader did not take them up and the order asks for none of them, and they stay
with the coordinator.

The fixer then ran, from the worktree's root with `BASE` set to `wo/050-agents-0.2.0`,
`python3 tools/check_docs.py` and `python3 tools/check_private_paths.py` (Errors: 0 each),
`git diff --check wo/050-agents-0.2.0...HEAD` (clean), and the text checks of "Done when", which
printed `0`, `0`, `agent-codex 0` and `agent-claude-code 0`, `0` each, `2`, `0`, `1` each, `1` each,
`2` each, `1` each, `1` each, `0` each, `15` each and `0`. On macOS arm64 with PID-scoped `ps`
allowed, in each package, `cargo fmt --check` and
`cargo clippy --offline --all-targets -- -D warnings` exited 0, and `cargo test --offline --release`
gave, for Codex, 15 wrapper tests passed and 0 failed, the library's unit tests 2, the manifest test
1 and the package test 1; for Claude Code, 15 wrapper tests passed and 0 failed, the library's unit
tests 2 and the package test 1. With `--nocapture`, `v2_failed_turn_fields_reach_the_result`
printed, in both packages, for `.v2` network
`client turn 1 failed: client failure: network; session cannot restart implicitly`, then
`{"failedTurn":{"cause":"client-failure","class":"network","clientStarted":true,"stopConfirmed":true,"survivors":[],"turn":1}}`,
and on the next call `session failed at turn 1: client failure: network; implicit restart refused`;
the same with class `authentication` for `FAIL:401 unauthorized`; for `.v1` network the first text
alone; and over the legacy handshake the same as the modern `.v2` network case.
`v2_client_tree_stop_leaves_no_survivor` took 10.23 s (Codex) and 10.58 s (Claude Code). The Codex
run printed the planted PIDs (client, background, session, double fork) deadline 85538, 85594,
85595, 85606 and cancellation 85870, 85881, 85882, 85893, each with the fields
`{"failedTurn":{"cause":"deadline","class":null,"clientStarted":true,"stopConfirmed":true,"survivors":[],"turn":1}}`
and the same with `cancelled`; and TERM 86397, 86408, 86409, 86420. The Claude Code run printed
deadline 86786, 86809, 86810, 86821; cancellation 87019, 87039, 87040, 87051, with the same fields;
and TERM 87288, 87308, 87309, 87320. Both gave the deadline, cancellation and `.v1` texts of order
050d word for word, and TERM's status 143 with empty stderr; `ps -o pid= -p` on the twenty-four PIDs
printed nothing. The Codex red check, the order's edit made on the statement F2 names, failed at
`tests/wrapper.rs:1593`, "v2 network result block count", 1 against 2; restored with `cp`, it passed
and `cmp` printed nothing. The package build printed
`dist/agent-codex.tar.gz#df657e25e670a7fd0f813dc9593099c6c8465993acf093b789acc45f0da00b12` and
`dist/agent-claude-code.tar.gz#3c67cbc7c4043bdaa2336c6213cc7caed221672c7034913b1d8b16f35f308a74`,
both new with F6; they are a host build's. The engine's expected failure,
`live_s1_scripted_agents_call_engine_with_distinct_run_identities`, was not run. The gate's run,
with that failure known, is the coordinator's, on the squash at order 050n's merge.

A second fresh read-only Opus reader read `b0532ca7` and found no must and four shoulds, which the
coordinator's fixer applied (S4 restoring `BASE`'s `check_report` helper and TERM call, so that
decision 6's "The TERM case does not change" holds), after which `cargo test --release` on macOS
arm64 gave, in each package, 15 wrapper tests passed and 0 failed,
`v2_client_tree_stop_leaves_no_survivor` included, the library's unit tests 2 and the package test
1, and for Codex the manifest test 1.
