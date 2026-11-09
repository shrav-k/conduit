# 008 — What a real client shows its model

Status: pre-registered on 2026-11-09, not yet run. This is action item A71 of the
[roadmap](../../ROADMAP.md#experiments-and-probes), which ruling 45 accepted from X5 of pull
request 10, from `allocmemory`, as E17 of round 6 run early. Ruling 48 added the cases that the
review of A70 found, and has a Codex worker build the harness under
[work order 002](../../docs/work-orders/002-real-client-harness.md). Under ruling 45's rule,
this page states what the experiment must show and how each result is recorded before
anything runs.

It runs no Conduit engine and is not conformance evidence. Its stub answers as the promises
say the engine's presented server answers, so that what a real client does with those answers
is seen before the engine exists.

## Question

When Claude Code and Codex, unchanged, reach the engine's presented server over loopback
Streamable HTTP with a bearer token, what does each client receive, what does it pass to its
model, and what does the model do next, when the server's answers are those of IFC-012,
IFC-013, IFC-025 and IFC-038?

## What it must show

For each client and case below: what reached the client on the wire, what the client gave its
model, and the model's next action, recorded apart (E17: "do not infer model visibility from
wire response alone"). In particular:

1. Whether a client's tool list follows the server's without a restart, when a Conduit closes
   and when one opens, which is the third thesis of
   [the first build](../../docs/implementation/FIRST_BUILD.md#the-theses-and-what-would-change-the-language).
2. What a model is told when a call is refused, fails, or ends with an unknown outcome, and
   whether any of Conduit's keys in `_meta` reach it.
3. Whether each client reads its token from a file by its header helper, and reads it again
   after a 401.

## The stub

A program in Rust, in this directory, that serves one MCP server at `http://127.0.0.1:<port>`
as the engine's presented server would, for one consumer:

- It admits a request only with `Authorization: Bearer <token>`, where the token is a signed
  JWT in the form of a JWT-SVID, minted at start with a local key and written to a file that
  the client's header helper reads (IFC-042, PRV-022's `token`). It can be told to answer the
  first request with 401, to see whether the helper is run again.
- It serves the tools of the open Conduits of a fixed scenario, named as IFC-012 names them,
  with `cacheScope: private` and `listChanged`, and notifies on a change (IFC-038).
- Its answers carry Conduit's keys in `_meta`, and `isError`, as IFC-025 says, and a call to a
  tool no longer presented is answered as IFC-012 and IFC-013 say.
- A control tool, `control.next`, moves the scenario to its next state: it closes or opens a
  Conduit, so that a change happens while the client's session runs.
- It logs every request and response, every notification and whether a stream was open to
  carry it, the protocol revision each client negotiates, and whether the token was valid. It
  never logs the token itself.

## Cases

| Case | The scenario | What the model is asked to do |
| --- | --- | --- |
| C1 | A Conduit is closed after the client has its list: the stub notifies, and the client calls the tool from its cached list | Call `files.read`, then `control.next`, then `files.read` again |
| C2 | A call to a tool that was never presented | Call a tool by a name the stub never listed |
| C3 | A tool fails: outcome `failed` | Call `files.read` on a path the stub fails |
| C4 | The outcome is unknown | Call `files.write`, which the stub answers with outcome `unknown` |
| C5 | A Conduit opens while the session runs: the stub notifies of a new tool | Call `control.next`, then use the new tool `notes.read` |

Each case runs with each client, under each variant that applies:

- Claude Code with its discovery cache on and off (`MCP_DISCOVERY_CACHE`), and Codex with its
  cached catalog on and off (`startup_readiness`).
- A `ttlMs` stated by the stub of 0, and of 60,000.
- The token: valid from the start, and 401 on the first request.

The model's behaviour varies between runs, so each cell is run three times. A wire fact needs
one run.

## Method

1. Work order 002 builds the stub and a runner in Rust, with tests that drive the stub by a
   scripted client and need no agent. The runner holds the prompts above, fixed before any
   run, and the matrix.
2. The admin session runs the runner. Each run starts the stub, starts the client headless in
   a throwaway lab directory outside the repository, configured to reach only the stub, and
   records the stub's log, the client's event stream and the model's final text.
3. The clients, as A70 ran them, keep the owner's configuration out: Claude Code with
   `--setting-sources`, `--strict-mcp-config` and `--mcp-config`, and only the stub's tools
   allowed; Codex with `--ignore-user-config`, `--strict-config`, `--disable apps` and a
   read-only sandbox. Each reads the token by its header helper: `headersHelper` for Claude
   Code, `http_headers_helper` for Codex.
4. A second work order classifies each run's next action, by these categories fixed now: the
   model retried the same call; told the user the call was refused, failed or unknown, in
   those words or the same meaning; called another tool; reported success that did not
   happen; or something else, quoted. Each classification cites the event that shows it.
5. The admin reviews the classification against the raw records before the results are
   written.

## Limits, stated before the run

- Headless only. Interactive sessions are not run unattended; a client that refreshes its list
  only when interactive will show as one that does not. A manual subset can follow.
- One machine, one version of each client: Claude Code 2.1.318 and Codex 0.167.0. Both change
  often, so the results are dated and name the versions.
- The stub answers as the promises say. Where it cannot, because a promise is not clear enough
  to build, that is a finding for the first build, logged as one.
- The models' behaviour is a sample of three runs per cell, not a measurement of a population.

## Results

None yet. Results go in this directory with their date, and the
[register](../../ROADMAP.md#experiments-and-probes) records the state.
