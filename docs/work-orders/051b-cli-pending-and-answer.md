# Work order 051b (C1b-1): `conduit pending` and `conduit answer`, with the unanswered hint

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share: this order adds no case, so `vectors` is an empty list. This order
has network access only because the shell's tests bind loopback ports, which the sandbox refuses
without it; use it for nothing else, add no crate, and reach no other machine. Run one cargo
command at a time. Never print a whole large file, case, fixture or log: use `head`, `tail`,
`grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or `export -p`, never print an
environment variable's value, and never read or print a key, a token or any credential. Do not
run `claude`, `codex` or any client, and sign in nowhere.

## What and why

**Where it sits.** [Order 117](117-plan-s2-s3.md)'s batch 1 gives "051b: the pending-action
commands `pending`, `sign-in` and `answer`. M. After 047 and 048b-join." [Spec batch 1's
plan](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/spec-batch-1/README.md) adds to it the client's URL elicitation (show the URL
and the message, ask consent, open the browser) and the unanswered hint, and the full review's
D53 adds the paste of a carried return where the command line runs on the node's machine and the
browser is elsewhere. That is three behaviors, so the coordinator cuts 051b into three, each
holding one ([FR-264](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/full-review/findings/P01-work-orders-lanes.md#fr-264): an
order holds one behavior and stays small; the coordinator's split, open to the owner's strike):

- **051b, this order**: the person sees and answers root's pending actions from the command
  line: `conduit pending`, `conduit answer`, and the hint that names `conduit pending` when a
  result counts unanswered actions. It needs only orders 047, 047b and 047c, all merged, and
  nothing of 048b-join.
- **051b-2**: `conduit sign-in`: the command line sends `sign-in.v1` declaring URL elicitation,
  shows the engine's message and the full URL, asks consent, opens the browser, and retries to
  the end. After 048b-join, and so after the cuts of 048a-5, 048a-6 and 048b-relay. It is the
  client A5's live check uses, since `conduit call` cannot answer an input-required result.
- **051b-3**: the carried return (PND-013, PND-014) from the command line: the paste read from
  the terminal with echo off where it runs on the node's machine and the browser is elsewhere
  (D53, ruling 75 C4's case), the loopback listener only where it knows it does not run there,
  and `conduit call` refusing to set `signInReturn`. After 051b-2, and so after 048a-8, whose
  redirect return it carries. The targeted review's
  [section 6](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/review-signin-2026-11-16/README.md#6-must-state-lists) says that this
  paste is the `carried: true` route and serves no Claude Agent's sign-in; 051b-3 carries that
  sentence.

[Ruling 82](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-82/README.md), point 3, has each dogfood slot's sign-in follow
051b; with the split, that is its last part, 051b-3, so a slot's sign-in follows all three parts
and the cut narrows nothing. [Order 117](117-plan-s2-s3.md) records the cut, and 050o follows
051b-3. This order builds no sign-in, no URL elicitation and no return.

**What it builds.** One behavior: root's pending actions, listed and answered from the command
line. PND-002: an unanswered, unexpired action "is raised at root and every Interface whose
scope contains a subject", "Raised means listed in list.v1's pending output and in inspect.v1's
status.pending for the subject, for such an Interface. Each result of that Interface's own
request carries io.github.shrav-k.conduit/unanswered, the number raised there, only when
nonzero." PND-009: `answer.v1` "is dispatched only on the Interface's own behalf, through an open
Conduit listing it to the target that authorizes inspection of the subject (ACC-033): the owning
Pool for a Resource, the owning Fleet for an Agent or a notice's Fleet, and management
otherwise, with the inspection fallback for a retired owner." [Ruling
50](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-50/README.md), point 3, makes the Interface a control panel where every
action waiting on a person is raised; the batch plan gives 051b the commands `pending` and `answer`
and the unanswered hint; the engine's own panel is 051c's. Reading `conduit pending` as ruling 50's
control panel on the command line is the coordinator's. The engine already does all of this; the command line only asks it and prints what
it answers.

**What orders 047, 047b and 047c bring, which this order assumes.** All three are merged on
`main`. From their Records: a failed or unknown setup raises a setup action whose ID is its
set-up.v1 invocation's (047); `list.v1` gives the actions raised at the caller in its `pending`
output, with the decision `list-pending-actions`, and each result of an Interface's own request
carries the `_meta` key `io.github.shrav-k.conduit/unanswered` where the count is not zero
(047b, `mark_unanswered` in `engine/crates/core/src/lib.rs`); `answer.v1` answers a setup action
`dismissed`, refuses under PND-009 an action not raised at the caller, reports invalid under
PND-009 an answer the kind does not take and conflict under PND-003 an answered or expired one,
routes through ACC-033's row 17, and writes a `pending-action-answered` record (047c,
`may_answer` in `pending.rs`). 047d (notices) is merged (#363) and 048a-2 (sign-in actions) is
not; root's `list.v1` may now give notices, and this order still routes the setup kind alone
(decision 3).

**What the shell already does, so that you do not redo it.** Line numbers are those of `main` at
`e4ceb4c8`; find each place by the text quoted with it. No shell source changed from there
through `3c761dd9` (047d), so they hold on `main` as you find it.

- `engine/crates/shell/src/main.rs`: `run` reads the command (`:18`, the usage string
  `"usage: conduit init|register|...`), collects positional arguments only for the commands
  listed at `:62` (`"call",` to `"unregister",`), refuses a command missing from the list at
  `:103` (`if ![`), and hands `install`, `submit`, `replace`, `retire` and `unregister` to
  `conduit_shell::author::run` (`:128`, `if ["install", "submit", ...`). `main` (`:440`) prints
  an error as `conduit {command}: {error}` and exits 1.
- `engine/crates/shell/src/author.rs`: `run` (`:168`) checks the positional count (`let expected
  = match command {`), then checks that `serve` holds the store before it sends anything, and
  matches the command. `route` (`:111`) gives the tool for an operation on `{kind, name}`: for a
  Resource or Agent whose owner is in state and not retired, `LOCAL.OPERATION`, `LOCAL` the
  local name of root's open Conduit to that owner listing the operation (an error where none
  does); `management.OPERATION` otherwise. That is ACC-033's rows 6, 6A and 7, the route
  PND-009 names for a setup action's subject. `call` (`:153`) sends one request through
  `server::client`, prints the result as one JSON line and fails where it did not succeed.
  `outcome` (`:40`) finds a `_meta` key by its suffix (`ends_with("/outcome")`).
- `engine/crates/shell/src/server.rs`: `client` (`:811`) sends one tool call or tool list as
  root (or `--as` for a list) over MCP and gives the result as JSON, `_meta` included; it ends
  `response.map_err(|error| format!(`.
- `engine/crates/shell/src/help.rs`: `COMMANDS` and `DESCRIPTIONS`, each of 12 entries, give
  every command's arguments, example and description.
- `engine/crates/shell/tests/serve.rs` has the helpers `cli` (`:61`), `start_server` (`:341`) and
  `stop_server` (`:391`); `engine/crates/shell/tests/cli.rs` has
  `help_describes_every_command_without_store_access` (`:404`) with its list `verbs`.
- No shell source mentions `pending` or `answer` as a command, and nothing prints the
  unanswered count.

**When this order starts.** Orders 047, 047b and 047c must be merged to `main`. Before you
change anything, check from the repository root that you find what they give, and stop, change
nothing and report if any check prints otherwise:

```bash
grep -c '"list-pending-actions"' engine/crates/core/src/records.rs
grep -c 'fn mark_unanswered' engine/crates/core/src/lib.rs
grep -c 'fn may_answer' engine/crates/core/src/pending.rs
grep -c '("answer.v1", "047c", "PND-009")' engine/crates/core/src/requests.rs
test -f conformance/conventions/setup/action-answers/case.yaml; echo "case $?"
grep -c '^## Record' docs/work-orders/047c-build-stored-answers.md
grep -c '"pending"' engine/crates/shell/src/main.rs
```

They print `1`, `1`, `1`, `0`, `case 0`, `1` and `0`. Orders that land beside this one (048a-3,
051a and others) touch the core or other commands; whichever lands second keeps both
changes. Where 051a or S0c has merged first and moved the commands into a table, add this
order's two commands to that table in the same places this order names, and say so.

**Who decided what.** PND-002 and PND-009, ACC-033's row 17, `list.v1`'s `pending` output and
IFC-025's `unanswered` key decide what is raised at root, what counts it and through which
target an answer goes; ruling 50, point 3, makes the Interface a control panel. Rulings 80 to
83 and the targeted review's section 6 put nothing on this part (ruling 83 touches only the Agent
packages and their `apiKey`): section 6 says "Nothing from this review changes 051b". Everything else here is the coordinator's, each open to the owner's
strike ([ruling 48](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-48/README.md)): the split into 051b, 051b-2 and 051b-3,
and this part's start after 047c alone; reading `conduit pending` as ruling 50's control panel on
the command line; the commands' names and arguments, which the batch plan
gives; printing each action as one JSON line, unchanged from `list.v1`; finding the action
through `list.v1` and routing its answer by its first subject through `route`; routing the setup
kind alone (the notice's Fleet is not in its view; a sign-in action is `sign-in.v1`'s); sending nothing for an action not raised at root; the
hint's text, its place on standard error once at the end, its source (the last result through
`serve`), and its absence after `conduit pending` and on the local paths of `register` and
`inspect`; the help texts; and the test. Apply them as written and report them under
`decisions` as the coordinator's.

**Read first:**

1. In `spec/`: PND-001, PND-002, PND-003 and PND-009 (`spec/promises/pending.yaml`); ACC-033's
   rows 6, 6A, 7 and 17 (`spec/conventions/access.yaml`); `list.v1` and `answer.v1`
   (`spec/core/operations.yaml`); `pending-action.v1` (`spec/core/results.yaml`); IFC-025's
   sentences on `pendingAction` and `unanswered` (`spec/interface/implementation.yaml`); the row
   "raised at" of `spec/definitions.md`.
2. The Records of [order 047b](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders/047b-build-actions-raised-by-scope.md) and
   [order 047c](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders/047c-build-stored-answers.md).
3. `engine/crates/shell/src/author.rs` and `main.rs` whole; `server.rs` from `pub async fn
   client`; `help.rs`.
4. In `engine/crates/shell/tests/serve.rs`, the test
   `author_commands_require_serve_and_make_recorded_requests` and the helpers above; in
   `engine/crates/shell/tests/changes.rs`, how a project's manifest gives an implementation
   `{"command": ["/test/program"], "implements": [...]}` and a template selects it as `"#NAME"`.

**Size: S-M**: about 0.2 to 0.3k changed lines of Rust with the tests, and one paragraph of
`engine/README.md`.

## The decisions it applies

1. **`conduit pending --store DIR`.** It takes no positional argument. In `author::run`, after
   the check that `serve` holds the store, it sends `management.list.v1` with `{}` through
   `server::client` (as root). Then:
   - where the result did not succeed (`succeeded` false): print it as one JSON line on standard
     output and fail with `listing root's pending actions through management.list.v1 did not succeed; see the printed result`;
   - otherwise, for each item of `structuredContent.pending`, in the order the engine gives
     them, print the item as one JSON line on standard output, unchanged; print nothing else on
     standard output. Where `pending` is absent or empty, print nothing on standard output and
     write to standard error the one line `conduit pending: no pending action is raised at root`.
     Exit 0.

   It lists root's actions alone: the command line holds only root's client token today (DL13),
   and another Interface's actions wait for its own token.

2. **`conduit answer --store DIR ACTION ANSWER`.** Two positional arguments: the action's ID and
   the answer, both passed as given. In `author::run`, after the check that `serve` holds the
   store:
   1. Send `management.list.v1` with `{}` and, where it did not succeed, print it and fail with
      `listing root's pending actions through management.list.v1 did not succeed; see the printed result`.
      Otherwise do not print it.
   2. Find in `structuredContent.pending` the item whose `id` equals `ACTION`. Where none does,
      send nothing more, print nothing on standard output, and fail with
      `no action ACTION is raised at root now: it is unknown, already answered or expired; nothing was sent. Run conduit pending to list the raised actions, or conduit call to ask the engine itself`,
      `ACTION` replaced by the argument.
   3. Give the item to `answer_subject` (decision 3). Where it gives an error, send nothing more
      and fail with that error.
   4. Otherwise `let tool = route(directory, &subject, "answer.v1")?;`, written as exactly that
      one line, `subject` the value `answer_subject` gave; where `route` errors, fail with its
      error. Then send `{"action": ACTION, "answer": ANSWER}` to `tool` through `author::call`,
      which prints the result as one JSON line and fails where it did not succeed.

   The answer is not checked here: the engine reports an answer the kind does not take as
   invalid under PND-009, and the command prints that result and exits 1.

3. **Routing by kind: `fn answer_subject(action: &Value) -> Result<Value>`**, in `author.rs`.
   Where `action["kind"]` is `"setup"` and `action["subject"]` is an array whose first item has a
   string `kind` and a string `name`, it gives `{"kind": K, "name": N}` of that first item: a
   setup action has one subject, the instance, and PND-009's target for it is ACC-033's rows 6,
   6A and 7, which `route` gives. Where the kind is `setup` and no such first subject exists, it
   gives the error `action ACTION names no subject; nothing was sent`. For any other kind, it
   gives the error
   `action ACTION is of kind KIND, and conduit answer routes only setup actions in this build; nothing was sent. Answer it with conduit call through root's Conduit to the target that PND-009 names for its kind, or with its kind's own command`.
   In both errors `ACTION` is the action's `id` and `KIND` its `kind`, each as the JSON string's
   text. A notice's target is its Fleet, and the action that `list.v1` gives names no Fleet: the
   engine keeps the notice's Fleet in its stored entry (`pending::notice_fleet`), so the command
   line cannot find root's Conduit to that Fleet. A sign-in action is answered by `sign-in.v1`
   alone. A later order adds the notice's route once the engine gives its Fleet to the caller (an
   open point for the coordinator; no order in the plan carries it yet).

4. **The unanswered hint.** In `author.rs`:
   - `pub fn unanswered_count(reply: &Value) -> u64` gives the value of the first `_meta` key
     ending in `/unanswered`, found as `outcome` finds its key, where that value is a
     non-negative integer, and 0 otherwise, including where `_meta` is absent. Nothing outside
     `_meta` counts.
   - A private `static UNANSWERED: std::sync::atomic::AtomicU64` starting at 0;
     `pub fn note_unanswered(reply: &Value)` stores `unanswered_count(reply)` in it (with
     `Ordering::SeqCst`), whatever its value, so that the latest result decides;
     `pub fn unanswered_hint() -> Option<String>` gives, where the stored count `N` is above 0,
     `conduit: N pending action(s) unanswered at root as of the last result; run conduit pending --store DIR to list them`,
     `N` as a decimal number and `DIR` written as those three letters, and `None` otherwise.

   In `server::client`, right before its final `response.map_err(`, where `response` is `Ok`,
   call `crate::author::note_unanswered` on it. Every request the command line sends through
   `serve` passes there, so the hint follows the last result it received. In `main.rs`'s
   `main`, after `run` returns and after any error line it prints, and before it exits, write
   `unanswered_hint()`'s text as one line on standard error where it gives one and the first
   argument is neither `pending` nor `__run-guard`. The exit status does not change. The local
   paths of `register` and `inspect`, which hold the store and do not go through `serve`, note
   nothing and print no hint; report this as a limit.

5. **The command line's lists and help.** In `main.rs`: add `pending|answer|` after `inspect|` in
   the usage string; add `"pending"` and `"answer"` to the list of commands that collect
   positional arguments, to the list of known commands, and to the list handed to
   `author::run`. In `author::run`, the expected count is 0 for `pending` (as for `inspect`)
   and 2 for `answer` (as for `replace`). In `help.rs`, `COMMANDS` and `DESCRIPTIONS` become 14
   entries each, the two new ones right after `inspect`'s, in this order:
   - `("pending", "", "pending --store /tmp/conduit")`, described
     `List the pending actions raised at root through serve, one JSON line each as list.v1 gives them; prints nothing when none is raised.`
   - `("answer", "ACTION ANSWER", "answer --store /tmp/conduit 00000000-0000-4000-8000-000000000001 dismissed")`,
     described
     `Answer a setup action raised at root through serve, through root's Conduit to the target that authorizes inspection of its subject; a setup takes dismissed. Sends nothing for an action not raised at root or of another kind.`

   In `engine/crates/shell/tests/cli.rs`, add `"pending"` and `"answer"` to the list `verbs` of
   `help_describes_every_command_without_store_access`, after `"inspect"`. Change nothing else
   in that file.

6. **Promise IDs in the shell's sources.** The shell's new text names no promise ID other than
   `PND-009`, which `engine/STANDING.md` already counts for the shell, so that the file does not
   change. If you name another, run `python3 tools/promise_standing.py --write`, as AGENTS.md
   asks, and say why.

7. **The unit tests**, in `author.rs`'s `mod tests`:
   - `unanswered_count_reads_only_its_meta_key`: `unanswered_count` gives 2 for
     `{"_meta": {"example.invalid/unanswered": 2}}`, and 0 for `{}`, for
     `{"_meta": {}}`, for `{"structuredContent": {"unanswered": 3}}`, for
     `{"_meta": {"example.invalid/unanswered": -1}}` and for
     `{"_meta": {"example.invalid/unanswered": "2"}}`. The key's prefix is neutral, since the
     code under `engine/` writes none of the names of `spec/names.yaml` (AGENTS.md), and it shows
     that the key is found by its suffix.
   - `answer_routes_setup_actions_only`: `answer_subject` gives `{"kind": "Resource", "name":
     "r"}` for `{"id": "a", "kind": "setup", "subject": [{"kind": "Resource", "name": "r", "id":
     "x"}]}`; an error containing `of kind notice` and `nothing was sent` for the same action
     with `"kind": "notice"`; and an error containing `names no subject` for the setup action
     with `"subject": []`.

8. **The test through `serve`**: `pending_and_answer_list_and_dismiss_a_failed_setup`, in
   `engine/crates/shell/tests/serve.rs`, a `#[test]`. It runs the `conduit` binary
   (`env!("CARGO_BIN_EXE_conduit")`) for every command, through a local helper that gives the
   `Output` without asserting its status, since `cli` asserts success and reads at least one
   line. In this order:
   1. Pick a free loopback port as `author_commands_require_serve_and_make_recorded_requests`
      does, run `init` with `--url http://127.0.0.1:PORT/mcp`, and `start_server`.
   2. Write a project with `conduit.json`
      `{"apiVersion": API, "address": "example.org/test/pending-answer", "implementations": {"local": {"implements": ["CORE#setup-hook.v1"], "command": ["/test/program"]}}}`
      and `documents.json` holding, in order: the Type `empty.v1` (`family: Resource`, `schema:
      {type: object}`, `offers: []`, `requires: []`); the Pool template `test-pool` of
      `CORE#pool.v1`; the Pool `workspace` from `test-pool`; the Resource template
      `failing-template`, `spec: {type: "#empty.v1", config: {}, implementation:
      {"CORE#setup-hook.v1": "#local"}}`; and the Resource `failing` from `failing-template` with
      owner `workspace`. `API` and `CORE` are read as the other tests read them. The program
      does not exist, so the Resource's setup cannot start its hook and ends failed or unknown,
      either of which raises its setup action (PND-006). If the loader refuses this shape, or
      the submission's setup step does not end failed or unknown, make the Resource's setup fail
      by the nearest means that keeps one submission, such as the Type listing `setup-hook.v1`
      under `requires`, and say so.
   3. `submit` the project: it exits non-zero, and its standard output holds the step line
      `{"step": "setup Resource/failing", ...}` with `outcome` `failed` or `unknown`.
   4. `pending`: exits 0; standard output is exactly one line, a JSON object whose `kind` is
      `setup`, whose `subject[0]` has `kind` `Resource` and `name` `failing`, and whose `id` is
      a string, `ID` below; standard error does not contain `unanswered`.
   5. `call management.inspect.v1 '{"object":{"kind":"Interface","name":"root"}}'`: exits 0, and
      standard error contains `conduit: 1 pending action(s) unanswered at root`.
   6. `answer 00000000-0000-4000-8000-0000000000ff dismissed`: exits non-zero, standard output is
      empty, and standard error contains `nothing was sent`.
   7. `answer ID retried`: exits non-zero; standard output is one JSON line whose `isError` is
      true and whose text contains `reports invalid under PND-009`.
   8. `answer ID dismissed`: exits 0; standard output is one JSON line whose `isError` is false.
      In `state.json`'s `state.records`, the last `invocation-dispatched` record whose
      `data.operation` is `CORE#answer.v1` has as `data.target` the ID of the Pool `workspace`,
      and a `pending-action-answered` record has `data.pendingAction` equal to `[ID]` and
      `data.answer.answer.value` equal to `dismissed`.
   9. `pending`: exits 0, standard output is empty, and standard error contains
      `no pending action is raised at root`.
   10. The call of step 5 again: exits 0, and standard error does not contain `unanswered`.
   11. `answer ID dismissed` again: exits non-zero, standard output is empty, and standard error
       contains `nothing was sent`.
   12. `stop_server(&mut server, &store, &[])`. If its check of run conditions fails because of
       the failed setup's run, stop the server with SIGTERM and wait for its exit as that
       helper's first loop does, and say so.

   Every assertion's message carries the command's standard error, as the file's other tests
   do, so that a failure says why; step 7's carry its standard output as well.

9. **`engine/README.md`.** In "Change a serving project", after the paragraph that begins
   "`conduit inspect --store DIR` works while serving" and before the `sh` block that follows
   it, add this paragraph, and change nothing else in the file:

   ```text
   `conduit pending --store DIR` lists, through `serve`, the pending actions raised at root,
   one JSON line each, as `list.v1`'s `pending` output gives them, and prints nothing on
   standard output when none is raised. `conduit answer --store DIR ACTION ANSWER` answers
   one of them through root's Conduit to the target that authorizes inspection of its
   subject (PND-009): a setup action takes `dismissed`. It answers only an action it finds
   raised at root, and only of the setup kind in this build; for any other it sends nothing
   and says so. After any other command whose last result through `serve` counts unanswered
   actions at root, the command line says how many on standard error and names
   `conduit pending`. The sign-in command is not built yet (order 051b-2).
   ```

10. **What this order does not do.** No promise, schema, definition, `IN_FORCE.md` line,
    ruling's record, case, fixture or core source changes. No `sign-in` command, URL
    elicitation, browser, paste or listener (051b-2, 051b-3); no route for notices or sign-in
    actions (decision 3); no client token for another Interface (DL13); no hint on the local
    paths. Report as findings, for the coordinator: whether the failed setup ended `failed` or
    `unknown` in your run and with which reason; whether `list.v1` at root gives notices in your
    run; whether the invalid answer of step 7 wrote an
    `invocation-dispatched` record; every place where a result reaches standard output without
    passing through `server::client`, and so updates no hint; and any place where PND-002,
    PND-009 or IFC-025 left you a choice this order does not settle.

## Scope

Change only `engine/crates/shell/src/author.rs` (decisions 1 to 4, 5's count and 7),
`engine/crates/shell/src/main.rs` (decisions 4 and 5), `engine/crates/shell/src/server.rs` (one
call in `client`, decision 4), `engine/crates/shell/src/help.rs` (decision 5),
`engine/crates/shell/tests/serve.rs` (decision 8's test), `engine/crates/shell/tests/cli.rs`
(decision 5's two names) and `engine/README.md` (decision 9's paragraph). `engine/STANDING.md`
changes only as decision 6 says. Not `spec/`, `IN_FORCE.md`, `tools/`, `packages/`,
`conformance/`, the core, the loader, `engine/CONFORMANCE.md`, the experiments,
`docs/implementation/` or `docs/proposals/`: report any defect as a finding.

## Done when

From `engine/`, one command at a time, each passes:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release -p conduit-shell --lib author::tests
cargo test --release -p conduit-shell --test serve pending_and_answer
cargo test --release -p conduit-shell --test cli help_describes_every_command
cargo test --release -p conduit-shell
CONDUIT_VECTOR_THREADS=4 cargo test --release
```

The third reports 4 passed and 0 failed: the two tests already there and decision 7's two. The
fourth and fifth each report 1 passed. Report the counts of the last two. If any other shell
test fails, stop and report it: do not change it. The threads are 4 because at 24 the runner ran
out of memory (order 071's Record).

Next, show that the test fails where the answer goes through `management`. From `engine/`, one
at a time:

```bash
cp crates/shell/src/author.rs target/author.rs.fixed
sed 's/let tool = route(directory, &subject, "answer.v1")?;/let tool = String::from("management.answer.v1");/' target/author.rs.fixed > crates/shell/src/author.rs
grep -c 'String::from("management.answer.v1")' crates/shell/src/author.rs
cargo test --release -p conduit-shell --test serve pending_and_answer
cp target/author.rs.fixed crates/shell/src/author.rs
cargo test --release -p conduit-shell --test serve pending_and_answer
```

The `grep` prints 1. The first test command must fail at step 7: its answer is refused under
ACC-033, since access is decided before PND-009's invalid (OPS-023), and so it does not report
invalid. Its failure message, which carries step 7's standard output, names `refused` or
`ACC-033`; a failure for any other reason does not count. After the restore the second reports
1 passed. Restore with `cp`, not `mv`: a moved file keeps its older
modification time, so cargo would not rebuild the crate. Git ignores `target/`. A warning that
`subject` is unused in the first build is expected.

From the repository root, each passes. `/usr/bin/python3` is the Mac's interpreter with PyYAML;
where `python3` has PyYAML, it serves as well.

```bash
python3 tools/check_docs.py
python3 tools/check_rulings.py
python3 tools/set_names.py --check
python3 tools/check_private_paths.py
python3 tools/promise_standing.py --check
git status --short -- spec IN_FORCE.md tools packages conformance engine/crates/core engine/crates/loader engine/CONFORMANCE.md experiments docs/implementation docs/proposals
git diff --check
```

`promise_standing.py --check` ends `Errors: 0`. The `git status` command prints nothing.

Each of these prints 1. The `tr` check reads the file with its line breaks as spaces, so a rewrap
hides nothing:

```bash
grep -c 'pub fn unanswered_count(reply: &Value) -> u64' engine/crates/shell/src/author.rs
grep -c 'fn answer_subject(action: &Value) -> Result<Value>' engine/crates/shell/src/author.rs
grep -c 'let tool = route(directory, &subject, "answer.v1")?;' engine/crates/shell/src/author.rs
grep -c 'note_unanswered' engine/crates/shell/src/server.rs
grep -c 'const COMMANDS: \[(&str, &str, &str); 14\]' engine/crates/shell/src/help.rs
grep -c 'fn pending_and_answer_list_and_dismiss_a_failed_setup' engine/crates/shell/tests/serve.rs
tr -s ' \n' '  ' < engine/README.md | grep -c 'lists, through `serve`, the pending actions raised at root'
```

This prints 0, since `engine/STANDING.md` does not change unless decision 6 says why:

```bash
git diff HEAD --stat -- engine/STANDING.md | wc -l
```

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark
each choice you made yourself as your own, give the last line of every check, each exit status
and count above, the signatures of decisions 3 and 4's functions, the exact standard error of
steps 5, 6 and 9 of the test, and the findings of decision 10.

`git diff --check` and `check_private_paths.py` read tracked files only, so the admin then stages
the new files and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`.
Before the merge the admin runs the local gate that [ruling 80](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-80/README.md),
point 10, has each merge wait on, `tools/gate.py`; CI runs on the pull request as well, and the
merge does not wait for it. A fresh Codex verifier and a fresh read-only Opus reader read the
result before it merges ([ruling 79](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-79/README.md), point 2). The admin writes
this order's Record at the foot of this file when it merges.

## Record

Run on 2026-11-18, from about 18:59 to 20:17 UTC, on the Mac from `main` `d891543f` (the order texts
of #366), in two rounds: the worker (2831 s) and a fresh Codex verifier (242 s), then the worker
again (1280 s) and a second fresh Codex verifier (299 s). The change is the seven files of Scope.
`conduit pending` and `conduit answer` join the command line's usage, its lists, the help's 14
entries and `author::run`; `author.rs` gains `answer_subject`, `unanswered_count`, `note_unanswered`
and `unanswered_hint`; `server::client` calls `note_unanswered` right before its final `map_err`;
`main` writes the hint after any error line; decision 7's two unit tests, decision 8's test through
`serve` and decision 9's paragraph of `engine/README.md` are as ordered. `engine/STANDING.md` is
unchanged: PND-009 is the only promise ID the new text names. The worker's findings: the failed
setup ended `failed` under OPS-022 (`Required part run ended failed.`); root's `list.v1` gave one
setup action and no notice; step 7's invalid answer wrote no `invocation-dispatched` record;
`init`'s first-start reply, `records`' export, the local `inspect` rows, the local `register`'s
replies and `submit`'s own step summaries reach standard output without passing through
`server::client`, and so update no hint; and the action that `list.v1` gives names no Fleet, so a
notice cannot be routed (decision 3's open point). Its own choices: one helper sends the list for
both commands, and a `pending` that is not an array is read as empty; a missing `id` or `kind` is
written `?` in decision 3's errors; and the unit test composes its neutral key from two literals,
since `set_names.py` flagged it as one.

The first verifier found one must: the global `--help` check and the option parsing caught a
flag-like ANSWER (`-dismissed`, `--help`) before the engine could judge it, against decision 2's
"passed as given". It found four shoulds: the dismissal's own hint was not checked; the unlisted
answers' checks could not tell an answer sent and discarded; the unit test's key outside `_meta`
lacked the `/unanswered` suffix; and step 7's parse failures dropped its standard output. Round 2
took the operands after `--store` as given, kept `conduit answer --help` alone as help, ran four
flag-like answers through `serve` to the engine's invalid under PND-009, checked that the
dismissal prints no hint, checked that an unlisted answer adds only `management.list.v1`'s access
decision, added a suffix-matching key inside `structuredContent` that counts 0, and carried all of
step 7's output into its parse failures. The second verifier found two musts and two shoulds:
`route` picks root's Conduit to the owner by a suffix match on `#answer.v1`, so where two Conduits
to one Pool list an `answer.v1` of different packages it can pick the wrong one; a successful
empty `pending` can write the session's initialization retry line before its own line; the
dismissal's no-hint check would pass if the list result never stored a count; and the flag-like
answers' checks would pass if the command line replaced each with `retried`.

A fresh read-only Opus reader then verified the order (ruling 79, point 2) at `8f280bc6`, on a
scratch copy of the tree that it and `origin/main` `8a393e49` (order 048a-3) merge to cleanly. It
ran every check of "Done when": `cargo fmt --check` and clippy (clean), the unit tests (4 passed,
0 failed), the test through `serve` and the help test (1 passed each), the shell suite (98 passed,
0 failed, 4 ignored) and the engine suite with `RUST_TEST_THREADS=4` (354 passed, 0 failed, 4
ignored); the management route's mutation, which failed at step 7 with `refused under ACC-033` in
the printed output and passed after the restore with `cp`; the five tools (Errors: 0 each), the
scope's `git status` (empty), `git diff --check origin/main...HEAD` (clean), the seven greps (1
each) and the `STANDING.md` count (0). It compared decisions 1 to 9 with the diff (word for word)
and the two commits' identity (neutral), scanned the added lines for private detail (none), and
probed `conduit answer`'s parse on a scratch store. It found no must and two shoulds:

- R1: the round-2 parser took every token after `--store` as an operand until it had two, so an
  option written between `--store` and ACTION (`--url`, `--call-timeout-seconds` and the others)
  was read as ACTION or ANSWER and the count check refused the command; and
  `conduit answer --store DIR --help` gave a count error instead of the help.
- R2: the second verifier's route must lies in the `route` this order requires and does not
  change, so it is not this diff's defect.

The fixer rebased the branch onto `origin/main` at `a701c1ee` (orders 204, 205, 050i and 050l)
with the work-order identity, without conflict, and did as follows:

- R1: `main.rs` takes only ANSWER, the token after ACTION, as given
  (`if command == "answer" && positional.len() == 1`); ACTION goes through the normal match, so an
  ACTION that begins with `-` is refused as an unknown argument and nothing is sent, a departure
  from decision 2's "both passed as given" that the coordinator accepted at the merge, open to the
  owner's strike (an engine-issued ID never begins with `-`), and `--help` anywhere else in
  `conduit answer` prints its help. The test drops the unlisted answer whose ACTION was `--help`
  and keeps the four flag-like ANSWER cases. On the built binary, `answer --store DIR --help`
  printed the help and exited 0, and `--url` or `--call-timeout-seconds` before ACTION, and `--url`
  as ANSWER, each passed the parse to `store is not initialized`. An option written between ACTION
  and ANSWER is read as ANSWER.
- R2 and the second verifier's first must: no change here. An open point for the coordinator: a
  later shell order chooses root's Conduit by the fully qualified `{core}#answer.v1` (the core
  package from `boundary::context`), with a regression test that opens two Conduits to one Pool.
- The second verifier's second must: no change to the code; this Record notes the exception, as
  that finding allows. `conduit pending`'s empty case writes its own one line; where
  `server::client` retries the session's initialization, as for every command through `serve`,
  its retry line comes before, and ruling 52 keeps that diagnostic visible.
- The second verifier's first should: the unlisted answer sent while one action is raised now
  checks the hint `conduit: 1 pending action(s) unanswered at root`, and the already-answered one
  at the end checks its absence, so a count the command line never stored fails the test.
- The second verifier's second should, in part: the engine's records carry no request arguments
  (an `access-decision` record holds the tool and no `arguments`), so the test cannot read the
  answer at the request boundary. Each flag-like answer now checks that its one access decision
  after the list is to `workspace.answer.v1`, so the parser did not swallow it; a replacement by
  another invalid string is still not detected.

The fixer then ran, from `engine/` with `RUST_TEST_THREADS=4` and `CONDUIT_VECTOR_THREADS=4`, on
the files of this commit before it was made: `cargo fmt --check` and
`cargo clippy --all-targets -- -D warnings` (exit 0 each), the unit tests (4 passed, 0 failed),
the test through `serve` and the help test (1 passed each), the shell suite (98 passed, 0 failed,
4 ignored) and the engine suite (354 passed, 0 failed, 4 ignored, exit 0). The mutation's `grep`
printed 1; the mutated test failed at step 7 with
`management.answer.v1 is refused under ACC-033` in its message, and after the restore with `cp` it
passed. From the root: `check_docs.py`, `check_rulings.py`, `set_names.py --check`,
`check_private_paths.py` and `promise_standing.py --check` (Errors: 0 and exit 0 each), the
scope's `git status` (empty), the seven greps (1 each), the `STANDING.md` count (0), and
`git diff --check origin/main...HEAD` (clean). The gate's run on this order's squash, which the
merge waits on, is the coordinator's at the merge.
