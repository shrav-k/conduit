# Experiment 008 — later-turn results of 2026-11-09

Evidence for A71 and E17 of round 6. This runs a stub, not a Conduit engine, and is not
conformance evidence. The [follow-up pre-registration](README.md#follow-up-a-later-turn-pre-registered-on-2026-11-09)
and [work order 008](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/work-orders/008-later-turn-results.md) fix its scope.
The admin supplied the run interval, 2026-11-09T22:13:41Z to 2026-11-09T22:15:59Z;
retained W timestamps locate the runs within it.

**All 12 runs completed both turns: 24 turns classified, none excluded.** Both clients used
`notes.read.v1` in turn two, in all three repetitions each. Claude Code's second-turn catalog
omitted the closed `files` tools, but its model still attempted the old read; the client
rejected it locally. Codex made no second-turn read attempt and reported the tool unavailable.
The four case tables below cite every run for these observations and counts.

These are the worker's classifications, reviewed by the admin before they were merged. Candidate findings are recommendations,
not accepted changes to the specification. The [first results](results-2026-11-09.md) measured
one turn; this follow-up measures the later turn of the same conversation.

## Evidence and classification

The [evidence index](evidence-2026-11-09-later-turn/README.md) describes the trimmed records.
Run labels use `CC` for Claude Code and `CX` for Codex; every run has cache off, `ttlMs` 0,
valid authentication and repetition r1, r2 or r3. Labels link `context.json`, which retains
version, variant, status, prompts and plans. A turn link contains its W/E/X/F records:
**W** means original `stub.jsonl`, **E** original `events.jsonl`, **X** original
`client-exposed.jsonl`, **F** complete `final.txt`. Numbers are one-based source lines in
that file and turn, not physical lines in the trimmed JSONL.

Wire evidence establishes emitted answers and requested methods, not model input. Claude's
init `tools` is a client catalog event, not the complete model request; user `message.content`
is conversation-shaped tool-result evidence. Assistant calls and final text show what the
model actually did and reported. Codex's MCP items are client-exposed results; its exact
function catalog and model-request payload are unavailable. In particular, a fetched list
alone does not prove the model received it. Successful notes calls and reports of the returned
text establish use independently of that inference. C1's absence report alone does not prove
what function declarations Codex supplied.

One exclusive category is assigned per turn using the original five categories. Turn one is
classified after the final prescribed control call: all models stopped and accurately reported
success, so these are **something else**, quoted. For C5 turn two, the next action after that
opening is the observed call to the newly available notes tool: **called another tool** means
that requested tool, not an unauthorized substitute. For C1 turn two, Claude reports an
observed local rejection (**told the user**); Codex reports mere unavailability without an
observed call or error (**something else**, as in the first results' missing-tool cases).
The newly requested C1 read is not a retry after a refusal. An offered retry is not an actual
retry. No new category for successful calls is introduced.

## Clients and execution

| Client | Installed version and observed model | Continuity and protocol | Runs |
| --- | --- | --- | --- |
| Claude Code | 2.1.318; `claude-sonnet-5-5` | One process with streamed user input; matching conversation ID across turns. Per-request discovery at revision `2026-07-28` in turn one; no reinitialization or list fetch in turn two. | [CC-C1-later-r1](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/claude-C1-later-cache-off-ttl-0-auth-valid-r1/context.json), [CC-C1-later-r2](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/claude-C1-later-cache-off-ttl-0-auth-valid-r2/context.json), [CC-C1-later-r3](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/claude-C1-later-cache-off-ttl-0-auth-valid-r3/context.json); [CC-C5-later-r1](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/claude-C5-later-cache-off-ttl-0-auth-valid-r1/context.json), [CC-C5-later-r2](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/claude-C5-later-cache-off-ttl-0-auth-valid-r2/context.json), [CC-C5-later-r3](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/claude-C5-later-cache-off-ttl-0-auth-valid-r3/context.json) |
| Codex | 0.167.0; `gpt-6.1-sol, low effort` | `exec` then `exec resume` with the same thread ID; two client processes. MCP initializes and negotiates `2025-06-18` in each turn, then fetches tools. | [CX-C1-later-r1](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/codex-C1-later-cache-off-ttl-0-auth-valid-r1/context.json), [CX-C1-later-r2](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/codex-C1-later-cache-off-ttl-0-auth-valid-r2/context.json), [CX-C1-later-r3](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/codex-C1-later-cache-off-ttl-0-auth-valid-r3/context.json); [CX-C5-later-r1](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/codex-C5-later-cache-off-ttl-0-auth-valid-r1/context.json), [CX-C5-later-r2](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/codex-C5-later-cache-off-ttl-0-auth-valid-r2/context.json), [CX-C5-later-r3](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/codex-C5-later-cache-off-ttl-0-auth-valid-r3/context.json) |

Plans retain the isolation flags, cache setting and fixed prompts; Codex's plans retain server
approval and omit `--ephemeral`, enabling the prescribed resume. Every pair's E startup
records retains the same anonymized conversation ID. These establish conversation continuity;
Codex's repeated W initialize requests establish a fresh MCP connection, not an uninterrupted
client process. Claude's E assistant events name its model; Codex's W call metadata names its
model and effort. See the six context links per client above and each turn's W/E records below.

## Summary by turn

Each row counts three repetitions; the corresponding case table below supplies their evidence.

| Client | Case | Turn | Retried same call | Told user refused/failed/unknown | Called another tool | False success | Something else | Excluded |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Claude Code | [C1-later](#claude-c1-later) | 1 | 0 | 0 | 0 | 0 | 3 | 0 |
| Claude Code | [C1-later](#claude-c1-later) | 2 | 0 | 3 | 0 | 0 | 0 | 0 |
| Claude Code | [C5-later](#claude-c5-later) | 1 | 0 | 0 | 0 | 0 | 3 | 0 |
| Claude Code | [C5-later](#claude-c5-later) | 2 | 0 | 0 | 3 | 0 | 0 | 0 |
| Codex | [C1-later](#codex-c1-later) | 1 | 0 | 0 | 0 | 0 | 3 | 0 |
| Codex | [C1-later](#codex-c1-later) | 2 | 0 | 0 | 0 | 0 | 3 | 0 |
| Codex | [C5-later](#codex-c5-later) | 1 | 0 | 0 | 0 | 0 | 3 | 0 |
| Codex | [C5-later](#codex-c5-later) | 2 | 0 | 0 | 3 | 0 | 0 | 0 |
| Total | All | Both | 0 | 3 | 6 | 0 | 15 | 0 |

## What changed between turns?

**Claude Code:** in all six runs, W shows a changed-list fetch immediately after the
turn-one notification, before the turn-two marker. There is no additional list request at the
turn boundary or during turn two. The second-turn init catalog reflects the changed list.
For C5 it includes `mcp__stub__notes_read_v1`, and assistant calls, conversation tool results
and final text show the model used it. For C1 it lists only `mcp__stub__control_next_v1`, yet
the model still emits `mcp__stub__files_read_v1`. The client supplies a user tool-result error,
“No such tool available”, and sends no read request to the stub. Thus the client stops
listing the tool, but the model still attempts it. Every run and its W/E lines are in
[Claude C1-later](#claude-c1-later) and [Claude C5-later](#claude-c5-later).

**Codex:** in all six runs, W shows no post-change list fetch during turn one. The first
process closes its notification stream and sends DELETE. After the turn-two marker, resume
initializes MCP afresh, opens a stream, and fetches the current list before any second-turn
MCP call. C5's list includes notes, and E records a successful notes call and final report.
C1's list contains only the control; E records no MCP call and no tool-result error, only
assistant text saying the read tool is unavailable. There is no public event showing an
explicit discovery operation supplied to the model. This is refresh on process resume of
the same conversation, not evidence of a live catalog update in a continuous Codex process.
See [Codex C1-later](#codex-c1-later) and [Codex C5-later](#codex-c5-later).

In all 12 runs W records a change with one notification stream open, successful delivery,
and the JSON notification on that stream. Those wire facts do not establish model visibility.
Claude fetches the current list in turn one; Codex fetches it only after resume. The case
rows retain the notification and request sequence for every run, including runs that send no
second-turn tool call.

## Classification of every run, per turn

### Claude C1-later

Turn one: W 9–23 includes the initial list, read success, control success, notification
and changed-list fetch; E 9/14 and 19/23 contain the two calls/results. Turn two: W has
only the turn marker and eventual stream closure; E 1 lists only the control, E 9 attempts
the old read, and E 10 delivers the local error. No protocol refusal reached the server.

| Run | Turn 1: evidence and next action | Turn 2: evidence and next action |
| --- | --- | --- |
| [CC-C1-later-r1](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/claude-C1-later-cache-off-ttl-0-auth-valid-r1/context.json) | [W/E/F](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/claude-C1-later-cache-off-ttl-0-auth-valid-r1/turn-1.jsonl); E 41, F: **Something else**: “Both calls succeeded, in the order you gave:  1. `files_read_v1` with path `hello.txt` returned `{"text":"Hello from memory."}`. 2. `control_next_v1` with `{}` returned `{"state":1}`.  I stopped after these two calls and used no other tools.” | [W/E/F](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/claude-C1-later-cache-off-ttl-0-auth-valid-r1/turn-2.jsonl); E 9–10: **Told the user**; F: “The read didn't happen. The call to `files_read_v1` failed with "No such tool available". The `stub` MCP server is connected but doesn't offer that tool right now, though the same call worked earlier in this session. I didn't try any other tool or a shell command in its place, so I have no new contents for `hello.txt`. The earlier read returned `{"text":"Hello from memory."}`.” |
| [CC-C1-later-r2](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/claude-C1-later-cache-off-ttl-0-auth-valid-r2/context.json) | [W/E/F](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/claude-C1-later-cache-off-ttl-0-auth-valid-r2/turn-1.jsonl); E 42, F: **Something else**: “Both calls ran in order and returned normally.  1. `files_read_v1` with path `hello.txt` returned `{"text":"Hello from memory."}`. 2. `control_next_v1` with `{}` returned `{"state":1}`.  I stopped after those two calls and used no other tools.” | [W/E/F](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/claude-C1-later-cache-off-ttl-0-auth-valid-r2/turn-2.jsonl); E 9–10: **Told the user**; F: “The read didn't happen. I called `files_read_v1` with path `hello.txt`, and it failed with "No such tool available". The `stub` MCP server is connected, but it doesn't offer that tool in this session, even though the same call worked earlier.” |
| [CC-C1-later-r3](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/claude-C1-later-cache-off-ttl-0-auth-valid-r3/context.json) | [W/E/F](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/claude-C1-later-cache-off-ttl-0-auth-valid-r3/turn-1.jsonl); E 40, F: **Something else**: “Both calls succeeded, in the order you gave:  1. `files_read_v1` with path `hello.txt` returned `{"text":"Hello from memory."}`. 2. `control_next_v1` with `{}` returned `{"state":1}`.  I made no other calls.” | [W/E/F](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/claude-C1-later-cache-off-ttl-0-auth-valid-r3/turn-2.jsonl); E 9–10: **Told the user**; F: “The second read of `hello.txt` didn't happen. `files_read_v1` is no longer available. The call returned an error saying the `stub` MCP server is connected but doesn't offer that tool here.” |

### Claude C5-later

Turn one: W 9–20 includes the initial list, control success, notification and changed-list
fetch; E 6/11 contains the control call/result. Turn two: E 1 includes notes, E 9 calls it,
E 13 supplies `{"text":"A newly opened note."}` as conversation content; W 2–4 records
the matching request and success. No new list fetch occurs in turn two.

| Run | Turn 1: evidence and next action | Turn 2: evidence and next action |
| --- | --- | --- |
| [CC-C5-later-r1](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/claude-C5-later-cache-off-ttl-0-auth-valid-r1/context.json) | [W/E/F](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/claude-C5-later-cache-off-ttl-0-auth-valid-r1/turn-1.jsonl); E 24, F: **Something else**: “I called `mcp__stub__control_next_v1` once with `{}`. It returned `{"state":1}`. I made no other calls, as you asked.” | [W/E/F](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/claude-C5-later-cache-off-ttl-0-auth-valid-r1/turn-2.jsonl); E 9/13: **Called another tool**, `notes.read.v1` on `hello.txt`; success, “A newly opened note.” |
| [CC-C5-later-r2](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/claude-C5-later-cache-off-ttl-0-auth-valid-r2/context.json) | [W/E/F](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/claude-C5-later-cache-off-ttl-0-auth-valid-r2/turn-1.jsonl); E 24, F: **Something else**: “I called `mcp__stub__control_next_v1` once with `{}`. It returned `{"state":1}`. I made no other calls, as you asked.” | [W/E/F](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/claude-C5-later-cache-off-ttl-0-auth-valid-r2/turn-2.jsonl); E 9/13: **Called another tool**, `notes.read.v1` on `hello.txt`; success, “A newly opened note.” |
| [CC-C5-later-r3](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/claude-C5-later-cache-off-ttl-0-auth-valid-r3/context.json) | [W/E/F](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/claude-C5-later-cache-off-ttl-0-auth-valid-r3/turn-1.jsonl); E 24, F: **Something else**: “I called `mcp__stub__control_next_v1` once with no arguments. It returned `{"state":1}`. I stopped there, as you asked, and used no other tools.” | [W/E/F](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/claude-C5-later-cache-off-ttl-0-auth-valid-r3/turn-2.jsonl); E 9/13: **Called another tool**, `notes.read.v1` on `hello.txt`; success, “A newly opened note.” |

### Codex C1-later

Turn one: W 12–29 includes the initial list, successful read/control, notification and
connection closure without another list fetch; E 4–7 records both calls/results. Turn two:
W 2–15 reinitializes and fetches only the control; no `tools/call` occurs. E 1 keeps the
thread ID, E 3 states intent to check/read, and E 4 reports unavailability, with no call
item or error result. A stated intention is not a measured attempted call.

| Run | Turn 1: evidence and next action | Turn 2: evidence and next action |
| --- | --- | --- |
| [CX-C1-later-r1](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/codex-C1-later-cache-off-ttl-0-auth-valid-r1/context.json) | [W/E/F](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/codex-C1-later-cache-off-ttl-0-auth-valid-r1/turn-1.jsonl); E 8, F: **Something else**: “`files.read.v1` read `hello.txt` and returned `"Hello from memory."` Then `control.next.v1({})` returned `{"state":1}`. Both succeeded; I stopped afterward.” | [W/E/F](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/codex-C1-later-cache-off-ttl-0-auth-valid-r1/turn-2.jsonl); E 4: **Something else**: “`files.read.v1` is no longer available, so I couldn’t read `hello.txt` again. No substitute tool was used.” |
| [CX-C1-later-r2](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/codex-C1-later-cache-off-ttl-0-auth-valid-r2/context.json) | [W/E/F](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/codex-C1-later-cache-off-ttl-0-auth-valid-r2/turn-1.jsonl); E 8, F: **Something else**: “`files.read.v1` read `hello.txt` and returned “Hello from memory.”  Then `control.next.v1({})` returned `state: 1`.  Both calls succeeded. Stopped afterward.” | [W/E/F](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/codex-C1-later-cache-off-ttl-0-auth-valid-r2/turn-2.jsonl); E 4: **Something else**: “`files.read.v1` is no longer available, so I couldn’t read `hello.txt` again. No substitute tool was used.” |
| [CX-C1-later-r3](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/codex-C1-later-cache-off-ttl-0-auth-valid-r3/context.json) | [W/E/F](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/codex-C1-later-cache-off-ttl-0-auth-valid-r3/turn-1.jsonl); E 8, F: **Something else**: “1. `files.read.v1` with `{"path":"hello.txt"}` returned `"Hello from memory."` 2. `control.next.v1` with `{}` returned `{"state":1}`.  Both succeeded. Stopped after these two MCP calls.” | [W/E/F](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/codex-C1-later-cache-off-ttl-0-auth-valid-r3/turn-2.jsonl); E 4: **Something else**: “`files.read.v1` is no longer available, so I couldn’t read `hello.txt` again. No substitute tool was used.” |

### Codex C5-later

Turn one: W 12–25 includes the initial list, control success, notification and connection
closure without another list fetch; E 4/5 records the control call/result. Turn two: W 2–15
reinitializes and fetches the list including notes, W 16–19 calls notes successfully;
E 4/5 records that call/result and E 6 reports its text. The exact model request remains
unavailable; the successful call is observed.

| Run | Turn 1: evidence and next action | Turn 2: evidence and next action |
| --- | --- | --- |
| [CX-C5-later-r1](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/codex-C5-later-cache-off-ttl-0-auth-valid-r1/context.json) | [W/E/F](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/codex-C5-later-cache-off-ttl-0-auth-valid-r1/turn-1.jsonl); E 6, F: **Something else**: “Called `control.next.v1` with `{}`. It returned `{"state":1}`. Stopped after that call.” | [W/E/F](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/codex-C5-later-cache-off-ttl-0-auth-valid-r1/turn-2.jsonl); E 4/5: **Called another tool**, `notes.read.v1` on `hello.txt`; success, “A newly opened note.” |
| [CX-C5-later-r2](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/codex-C5-later-cache-off-ttl-0-auth-valid-r2/context.json) | [W/E/F](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/codex-C5-later-cache-off-ttl-0-auth-valid-r2/turn-1.jsonl); E 6, F: **Something else**: “Called `control.next.v1` with `{}`. It returned `{"state":1}`. Stopped after that call.” | [W/E/F](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/codex-C5-later-cache-off-ttl-0-auth-valid-r2/turn-2.jsonl); E 4/5: **Called another tool**, `notes.read.v1` on `hello.txt`; success, “A newly opened note.” |
| [CX-C5-later-r3](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/codex-C5-later-cache-off-ttl-0-auth-valid-r3/context.json) | [W/E/F](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/codex-C5-later-cache-off-ttl-0-auth-valid-r3/turn-1.jsonl); E 6, F: **Something else**: “Called `control.next.v1` with `{}`. It succeeded and returned `{"state":1}`. Stopped after the call.” | [W/E/F](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/codex-C5-later-cache-off-ttl-0-auth-valid-r3/turn-2.jsonl); E 4/5: **Called another tool**, `notes.read.v1` on `hello.txt`; success, “A newly opened note.” |

## Runs not as planned, deviations and limits

None is excluded. Each of the 12 context records linked above has `runner_error: null`,
process exit 0, no timeout, and two completed successful turn statuses. E contains a result
or turn-completed boundary for both turns in every run. The local C1 rejection is an
experimental outcome, not a failed run. All prescribed first-turn calls and C5 second-turn
calls succeeded; the quoted reports agree with W/E. No observed action retried after a
rejection, used a substitute or falsely claimed a new read had succeeded.

The records match the pre-registered 12-run matrix and the prescribed continuation
methods. No execution deviation was found. As in the first work order, the worker wrote the
classifications before the admin's review. The admin reviewed them afterwards and before
merging, on 2026-11-09: it read the second turn's final text of all 12 runs, and the worker's
and a fresh verifier's reports, which found no fault; each agrees with the classification.

The [pre-registered limits](README.md#limits-stated-before-the-run) still apply: headless,
one machine, one version/model combination per client, a stub with no engine, and three
repetitions per cell rather than a population measurement. Plans remove API-key environment
variables and request the established isolated subscription setup; these records do not
independently establish the model-service account identity. No credential was inspected.

Only cache off and `ttlMs` 0 were tested in this follow-up, as the context cells show. There
is no measurement here of cache-on behavior, TTL expiry, interactive sessions, a third turn,
unsolicited use of the new tool, or a continuous Codex process receiving a second user turn.
Codex resume starts a new process and MCP connection, unlike Claude's streamed continuation;
therefore these results cannot isolate a model effect from client lifecycle or model choice.
Both second-turn prompts name the requested tool explicitly; C5 also asks for discovery if
the client permits it. The evidence establishes prompted use and closure handling, not
spontaneous discovery. Exact model-input catalogs and raw metadata visibility remain
unavailable, as distinguished in the case tables.

## Candidate findings for the admin

These bear on [IFC-012, IFC-013 and IFC-038](../../spec/interface/implementation.yaml)
and [the third thesis](../../docs/implementation/FIRST_BUILD.md#the-theses-and-what-would-change-the-language).
They require no change to `spec/` in this work order.

1. **Opening: usable on the next turn in the tested continuation modes.** All six
   [C5-later rows](#claude-c5-later), including [Codex's three](#codex-c5-later), show real
   successful notes calls. This qualifies the first results' same-turn limitation: Claude's
   fetched list becomes usable on a later streamed turn; Codex's resumed process fetches it
   anew. Recommend keeping within-turn refresh, later streamed turns and process resume
   separate when assessing the third thesis. This does not establish uninterrupted native
   Codex refresh or justify a promise that any client updates at any point.
2. **Closing: catalog omission and attempted use remain separate.** All three
   [Claude C1-later rows](#claude-c1-later) show a catalog without files followed by an old-name
   attempt and a local rejection; all three [Codex C1-later rows](#codex-c1-later) show a
   fresh list and no attempt. No closed-tool request reaches the stub in turn two, so this
   follow-up adds no engine-side refusal observation for IFC-012/IFC-013. Recommend retaining
   current authorization checks and testing both local rejection and server refusal; omission
   from a client catalog alone is not evidence that a model will never emit the old name.
