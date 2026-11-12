# Work order 142 (SCR1): the core scripted MCP Agent client

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access, to fetch crates and
for tests that bind a loopback port; use it for nothing else, and reach no real service. Start no
real Codex or Claude client and use no login. Run one cargo command at a time, and never print a
whole large file or log. Never run `env`, `printenv`, `set` or `export -p`, never print an
environment variable's value, and never print a key or a token, even a synthetic one that a test
made.

## What and why

[S4's plan](../../experiments/015-s4-partial-setup/plan.md#the-orders) splits its scripted
client in two: SCR1, this order, and SCR2, a later one (the row SCR1 and the paragraph
"SCR1 / SCR2"). SCR1 is the core scripted MCP Agent client, shared apparatus rather than a
numbered experiment: it initializes MCP, discovers tools, authenticates only as its supplied
Agent identity, sends keyed, fresh and unkeyed calls, holds acknowledged barriers and records
redacted request and reply traces. S4-H will drive S4's tasks and D02's twelve matrix cells
through it, and S2-H will drive S2's through SCR2
([S2's plan](../../experiments/013-s2-shared-login/plan.md), the paragraph "S2-H"). The plan sizes
it M, 350 to 550 lines; with its tests it will be larger, about L.

It runs as an Agent: the Codex package's wrapper, `packages/agent-codex/`, launches it as its
client program through `--client-program PATH`, a scripted-test injection that the package's
item does not carry. It speaks the protocol mode that the wrapper reads from its client, and
emulates nothing else of Codex: no model and no native tools. The plan says SCR runs "through the
Codex package's `scripted-client` binary"; that binary is the package's own test fixture,
`tests/fixtures/client.rs`. This order reads the sentence as running SCR through that package's
wrapper as its client program, and leaves the fixture as it is.

It sends exactly the requests that a script names, as the Agent whose run launched it, and
records what it sent and what came back, so that a harness can count, for each matrix cell, the
key supplied or its absence, the calls on the wire, the invocation IDs and each outcome or reason
([S4's README](../../experiments/015-s4-partial-setup/README.md#measures), M1). Repeats are the
script's: the client never sends a request again on its own.

Read first:

- S4's plan: the row SCR1, the paragraphs "SCR1 / SCR2" and "S4-H", and the sentence above the
  table on tools checks; S4's README: the preparation paragraph of "The tasks", M1 and "Runs";
- S2's plan, the paragraph "S2-H", and P2 in
  [S2's README](../../experiments/013-s2-shared-login/README.md#what-it-must-show): simulated
  clients alone do not pass;
- `packages/agent-codex/`: [its README](../../packages/agent-codex/README.md) (the runner's
  inputs, the header helper and `--client-program`); in `src/lib.rs`, `arguments` (the argv of a
  first and of a resumed turn), `helper_command`, `header` and `Session::run_turn` (what it writes
  to the client's stdin and which events it reads); `src/main.rs`; `tests/fixtures/client.rs`
  (how a client parses that argv, runs the helper and reaches the engine over Streamable HTTP with
  rmcp); and `tests/wrapper.rs` (how a test launches the wrapper with a client program);
- IFC-022, IFC-023, IFC-025 and IFC-036 in `spec/interface/implementation.yaml`; OPS-009,
  OPS-010, OPS-011 and OPS-032 in `spec/conventions/operations.yaml`; `metaPrefix` in
  `spec/names.yaml`.

## The decisions it applies

1. **Where it lives.** `experiments/015-s4-partial-setup/scripted-client/`, a crate named
   `scr-client` with one binary of the same name, its own `Cargo.toml`, committed `Cargo.lock`,
   `rust-toolchain.toml` at 1.97.1 and a `.gitignore` for `target/`. It depends on no crate of
   this repository, not even as a dev-dependency: its launch test (decision 10) runs a separately
   built `agent-codex`. It uses the versions that agent-codex pins for rmcp (`=3.9.0`) and reqwest
   (0.13). Its binary is named apart from agent-codex's fixture binary, `scripted-client`.
2. **The launch.** It accepts the argv that agent-codex's `arguments` builds for a Codex launch,
   as the fixture parses it: first `exec`, last `-`, with exactly one
   `-c mcp_servers.engine.url=JSON` and one `-c mcp_servers.engine.http_headers_helper=JSON`,
   each a JSON string. `resume` makes the turn a resumed one, whose thread ID is the argument
   before `-`. It ignores the wrapper's other options. Any other argv, the Claude Code adapter's
   `--input-format stream-json` form among them, ends with exit status 2, nothing on stdout and
   no request. It reads the turn's message from stdin to its end. It writes to stdout only three
   JSON lines, each flushed: `{"type":"thread.started","thread_id":ID}`, first in every turn it
   accepts, with no `tools` key, so the wrapper reports the client's catalog as `not_reported`;
   then, once the script has ended,
   `{"type":"item.completed","item":{"type":"agent_message","text":SUMMARY}}` and
   `{"type":"turn.completed"}`, and it exits 0. A first turn's ID is `scr-` followed by a new
   UUID; a resumed turn repeats the ID from its argv, as the wrapper requires. It writes no
   request or reply to stderr. The wrapper ends a turn after 180 seconds and poisons the
   conversation, so a script must finish within that.
3. **Authentication only as its Agent** (IFC-022). It reads no environment variable and no file but
   its stdin and its own barrier files, and nothing under the home directory: no owner configuration
   and no credential. Before its first request it checks the URL from its argv: only `http` to a
   loopback address. An `https` engine address, as an isolated run's would be, needs TLS that it
   does not build now: report that, for the order that first needs it. It then runs the helper
   command from its argv once, as the fixture does for Codex: `sh -c HELPER` with a cleared
   environment, stdin closed and stderr discarded. The helper's stdout must be one JSON object
   holding only `Authorization`, whose value is `Bearer ` followed by a non-empty token. It keeps
   the token in memory and sends it only as that header, only to that URL. It never writes the
   token, the header or the helper command anywhere. A URL it refuses, a helper that fails or one
   that prints anything else stops the script with nothing sent, the first without running the
   helper. It is given no other way to present a credential.
4. **The script.** The turn's message is one JSON object at version `"scr": "v1"`; an unknown
   field, version or step makes it invalid. Tool names below are illustrative:

   ```json
   {
     "scr": "v1",
     "id": "t3-cell-01",
     "trace": "t3-cell-01.jsonl",
     "barrierDir": "/path/to/barriers",
     "redact": ["synthetic-marker-0001"],
     "steps": [
       {"step": "discover"},
       {"step": "call", "tool": "effect.write.v1", "arguments": {"entry": 1},
        "mode": "keyed", "key": "cell-01-a"},
       {"step": "barrier", "name": "first-held"},
       {"step": "call", "tool": "effect.write.v1", "arguments": {"entry": 1},
        "mode": "keyed", "key": "cell-01-a"},
       {"step": "call", "tool": "effect.write.v1", "arguments": {"entry": 1}, "mode": "fresh"},
       {"step": "call", "tool": "effect.write.v1", "arguments": {"entry": 1}, "mode": "unkeyed"}
     ]
   }
   ```

   - `id` and `trace`: one to 64 and one to 128 characters of `[A-Za-z0-9._-]`, starting with a
     letter or digit; `trace` names the trace file (decision 8).
   - `barrierDir`: required where a step is a barrier (decision 7).
   - `callTimeoutMs` and `barrierTimeoutMs`: optional, 1 to 170000, each 60000 by default; and
     the whole script has a deadline of 170 seconds, past which it stops with
     `script-deadline` and still completes the turn, inside the wrapper's 180.
   - `redact`: optional synthetic markers, each at least 8 characters (decision 8).
   - `steps`: one to 256. `discover` lists the tools. `call` names a `tool`, `arguments` (an
     object, `{}` by default) and a `mode`, `keyed`, `fresh` or `unkeyed`, with `key`, a
     non-empty string, present exactly when the mode is `keyed`. Each call waits for its reply
     before the next step. `barrier` has a `name`.

   The whole script is checked before anything is created or sent. An invalid one creates no
   file, runs no helper, sends nothing and completes the turn with status `invalid` and a
   `problem` that names the field, never its value.
5. **Calls and their `_meta`** (IFC-023, IFC-036, OPS-011). A `keyed` call's `_meta` holds
   `io.github.shrav-k.conduit/idempotencyKey` with the given key, and declares Conduit's
   extension: its `io.modelcontextprotocol/clientCapabilities` is the session's own capabilities
   with `extensions` naming `io.github.shrav-k.conduit/implementation-interface` at `revision`
   `v0`. A `fresh` call is the same with a new key, `scr-` and a UUID v4, made for that step and
   recorded. An `unkeyed` call carries no key of Conduit's and no declaration, as a client that
   knows nothing of Conduit sends it. Arguments are sent as the script gives them and never carry
   the key. The script makes the repeats of OPS-009, OPS-010 and OPS-032: the same key, tool and
   arguments for a repeat, and the same key with other arguments for a conflict. A repeat sent
   while the first call is still pending (OPS-032) needs calls that do not wait for their
   replies: that is SCR2's, with the server's hold that tests it.
   The client judges no result. A call names a tool that the turn's latest `discover` listed;
   otherwise the script stops with `undiscovered-tool` and nothing is sent for it (calling a
   cached name that the list no longer holds is SCR2's). Write the prefix and the extension's
   name once, in one module, with a comment naming their sources: `metaPrefix` in
   `spec/names.yaml`, and the extension's name in `spec/definitions.md`, the row "Conduit's
   extension";
   `tools/set_names.py` does not reach `experiments/`: report that. rmcp 3.9.0 stamps
   `io.modelcontextprotocol/clientCapabilities` on a request only at revision 2026-07-28, and
   `PeerRequestOptions::meta` then replaces a key it names whole: so a keyed or fresh call
   supplies the complete capabilities object, the session's own with the extension added. The
   tests check the `_meta` that the server received, not the one the client built.
6. **No hidden retries.** A `call` step sends exactly one `tools/call`.
   A `discover` step sends one `tools/list` per page, following `nextCursor`, at most 16 pages.
   The session opens once per turn, before the first `discover` or `call` step, with the startup
   exchange that rmcp 3.9.0 makes, and names itself `scr-client`, at its crate version, in its
   client information; a failure to open stops the script. No request is ever sent again, and
   none is sent that the script does not name:
   - every request goes through `Peer::send_request_with_option`, with
     `ClientRequest::CallToolRequest` or `ClientRequest::ListToolsRequest`, the client waiting on
     the returned handle itself, whose drop cancels nothing; never `call_tool`, `call_tool_once`,
     `list_tools` or `list_all_tools`, which send `tools/call` again on `InputRequired`, send
     `notifications/cancelled` when dropped, or answer `tools/list` from a cache, a stale one
     after a failure;
   - the HTTP client is reqwest built with `retry(reqwest::retry::never())`, since its default
     retries protocol NACKs, with no redirects and no proxy;
   - the transport sets `reinit_on_expired_session` to false, since rmcp's default initializes
     again after an HTTP 404 and sends the POST again, and its `retry_config` is
     `Arc::new(NeverRetry::default())`, assigned to the field, since the struct is
     non-exhaustive and has no builder for it;
   - the client bounds each wait itself, not through `PeerRequestOptions::timeout`, which sends
     `notifications/cancelled` when it expires; the client sends no cancellation.

   A result, a JSON-RPC error or a transport failure is recorded and the script goes on; it is
   the script's to repeat. A call with no reply within `callTimeoutMs` stops the script, as does
   a failed `discover`, the first or a later one. At the end the client closes the session and
   cancels nothing still pending.
7. **Barriers, with an explicit handshake.** `barrierDir` is an absolute path to an existing
   directory owned by the user the client runs as and writable by no group or other user; otherwise
   the script is invalid. A barrier's `name` is a lowercase letter or digit followed by up to 63
   lowercase letters, digits or hyphens, unique in the script, and none of its files may exist when
   the script starts. At a barrier the client: writes `barrier-reached` to the trace; creates the
   empty file `NAME.reached` (new, 0600); waits until `NAME.release` exists, polling for it; writes
   `barrier-released` to the trace; creates `NAME.acknowledged`; and only then goes to the next
   step. A harness releases a barrier by creating `NAME.release`, and knows that the client has
   passed it once `NAME.acknowledged` exists. No barrier past `barrierTimeoutMs` is passed: the
   script stops with `barrier-timeout`. The files are the handshake: nothing waits a fixed time to
   order events, in the client or in a test, which waits for a file or a server's count.
8. **The trace, the summary and redaction.**
   - **Trace:** the file named by `trace` in the client's working directory, which is the run's
     private working directory that the runner supplies to the wrapper. It is created new with
     mode 0600; an existing file makes the script invalid. JSON Lines, one event a line, each
     written and flushed when it happens, so that a client the wrapper ends leaves a trace with
     no `end` line. Each line has `seq`, from 1 without gaps, `at` (UTC, RFC 3339, milliseconds),
     `elapsedMs` (monotonic, since the client started), `event` and, where one applies, `step`.
     The events: `open` (the negotiated protocol revision and the server's information),
     `request` (the method and the params sent, with the `_meta` keys the client set), `reply`
     (the result, the error's code, message and data, or the transport failure's kind, with the
     HTTP status where the transport reports one), `timeout`, the two barrier events, and `end`
     (the status). It holds no header, token, helper command or `redact` list. The summary
     names the file; how a harness collects it from that directory is S4-H's.
   - **Summary:** the agent message's text, one JSON object: `scr`, `id`, `status`
     (`completed`, `stopped` or `invalid`), `trace`, `stoppedAt`, `cause` (`url-refused`,
     `token-unavailable`, `open-failed`, `discover-failed`, `undiscovered-tool`, `call-timeout`,
     `barrier-timeout` or `trace-failed`), `problem` and `steps`. Each step gives its `index`
     and kind; `discover` its tool names; `call` its tool, mode, key or null and label; each
     reply `reply` (`result`, `error`, `transport`, `timeout` or `pending`), `isError`, the
     values of IFC-025's keys `outcome`, `reason`, `restriction` and `invocation`, the `ruleId`s
     of `structuredContent.diagnostics`, and an error's code; `barrier` whether it was
     acknowledged. It copies what came back and judges nothing. A stopped or invalid script
     still completes the turn, so that the caller reads why (ruling 52), where a failed turn
     would give it only the wrapper's fixed text.
   - **Redaction:** every trace line, the summary and anything written to stderr pass one
     redactor before they are written, which replaces the token and each `redact` marker, raw
     and JSON-escaped, with `[redacted]`.
9. **Its README** says first that it is shared apparatus for S4, and for S2 through SCR2, not a
   client for general use, and never a substitute for a real client where one is required (S2's
   P2, S4's E09 replay). It gives the launch through agent-codex's `--client-program`, which a
   harness's own Agent item must add, since the package's item leaves it out; the script, trace
   and summary forms; the barrier handshake; the 180-second turn; where its names come from; and
   that a change of agent-codex's argv, as 050 may make, fails the launch test and needs a change
   here.
10. **Tests.** The scripted server is rmcp's Streamable HTTP server, started in process on a
    loopback port behind a layer that answers 401 without the expected bearer, records each
    POST's JSON-RPC method, `_meta` and arguments as received, and can answer a chosen
    `tools/call` with HTTP 404 or 500 instead of passing it on. Its tools answer crafted results
    with IFC-025's keys. Every direct test runs the binary with a cleared environment, `HOME` in
    a temporary directory and a planted sentinel variable, as agent-codex's `tests/wrapper.rs`
    does, and asserts that the sentinel appears nowhere. The tests drive the built
    binary (`CARGO_BIN_EXE_scr-client`) with the argv that agent-codex's `arguments` builds
    today, copied into the test, not imported, and a helper command that prints a header file
    the test writes. Tokens and markers are synthetic, in no real token's format, and compared
    with `assert!(a == b)` or `assert!(!text.contains(marker))`, never `assert_eq!`, where one
    could print:
    - launch: a first and a resumed turn each give exactly the three lines, the resumed one
      repeating its thread ID; the Claude Code form, an argv without the final `-` and one
      without either `-c` setting each exit with status 2, nothing on stdout and no request;
    - authentication: every POST carries the helper's token; a failing helper and a malformed
      header each stop with nothing sent; an `http` URL to a non-loopback address, such as
      `192.0.2.1`, and an `https` URL each stop without running the helper;
    - discovery over two pages, both traced; two `discover` steps send two `tools/list`; a
      failed second `discover` stops the script; a call to a tool not listed stops, sending
      nothing;
    - `_meta`: a keyed call carries its key and the declaration, with the session's other
      capabilities kept; two fresh calls carry two different keys, each the one recorded; an
      unkeyed call carries no key and no declaration; arguments arrive unchanged;
    - exactly once: each call step is one POST, including when the server answers 500 or a
      JSON-RPC error, and the script goes on after each; and when it answers 404, both at the
      default revision and with the server at revision 2025-11-25 in `legacy_session_mode`,
      where a 404 counts as an expired session, with no new `initialize` or `server/discover`
      after it;
      a call held past `callTimeoutMs` stops the script, with no cancellation and no later
      request; and a response stream closed before its reply, where you can arrange one (your
      choice; report it);
    - barrier order: the example's first four steps; the test waits for `first-held.reached`, sees
      no request after the first call's, then creates `first-held.release`; the server receives the
      repeat only after `first-held.acknowledged` exists, and the summary gives both replies. A
      barrier never released stops with `barrier-timeout` and no later request; a script past its
      170-second deadline, by a shortened deadline in a test build (your choice how; report it),
      stops with `script-deadline` and completes the turn; a barrier file present at the start, and
      a `barrierDir` writable by its group, each make the script invalid, with nothing sent;
    - the trace: mode 0600 in the working directory; an existing file, and a name with `/` or a
      leading `.`, make the script invalid; `seq` has no gaps; IFC-025's keys and the `ruleId`s
      reach the summary unchanged;
    - redaction: the token, and markers that the server returns in `content`, in
      `structuredContent` and in an error's message, and one sent in arguments, raw and
      JSON-escaped (one marker holds `"` and `\`), appear in no trace line, summary, stdout or
      stderr;
    - the launch through agent-codex: `SCR_AGENT_CODEX` names a built `agent-codex` binary.
      Where it is unset, the test prints that it skipped and why, and passes; where it names no
      file, the test fails. With it, the test launches the wrapper as `tests/wrapper.rs` does:
      `--client-program` naming this crate's binary, a cleared environment with `PATH`, `HOME`
      set to a temporary directory, `CONDUIT_URL` set to the server,
      `CONDUIT_IDENTITY_DIR` set to a temporary directory whose `token` holds a synthetic token,
      and `AGENT_ACCOUNT_SERVICES` and `AGENT_PEER_MESSAGING` false. Through an rmcp stdio client
      it calls `turn` twice, with two scripts whose `trace` and barrier names differ: both
      succeed with status `completed`, the second as a resumed turn; the
      result's text is the summary and its `clientTools` are `not_reported`; every POST carried
      the token from `token`, read by agent-codex's own `--headers-helper`; the trace in the
      wrapper's working directory has mode 0600; and neither result nor the wrapper's stderr
      holds the token.
11. **What this order does not do:** SCR2's task drivers, calls that do not wait for their
    replies and the repeats in flight they send (OPS-032), S2's keyed replay after a deadline,
    forced cached tool names and serving IDs, retained-output assertions and clock
    synchronization; S4-H's fault scheduling and proxy; the Claude Code launch form; any change
    to `packages/agent-codex/`, `engine/`, `conformance/`, `spec/` or a manifest; any model work
    or native-client emulation beyond the protocol mode; and any evidence of a real client's
    behavior.

## Scope

Change only `experiments/015-s4-partial-setup/scripted-client/` (new).

## Done when

First, in `packages/agent-codex/`, `cargo build --release --locked --bin agent-codex`, which
writes only that crate's ignored `target/`. Then, in the new crate:
`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test --release`
twice: once without `SCR_AGENT_CODEX`, when the launch test says it skipped, and once with it
set to that binary's absolute path, when the launch test runs. From the repository root, the
tools checks that S4's plan names: `python3 tools/check_docs.py`,
`python3 tools/check_rulings.py`, `python3 tools/set_names.py --check`,
`python3 tools/check_schemas.py`, `python3 tools/check_roadmap.py`,
`python3 -m unittest discover -s tools/tests`, with the interpreter that has PyYAML, and
`git diff --check`. Run these with `bash`. Each prints nothing when it passes:

```bash
D=experiments/015-s4-partial-setup/scripted-client
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]|/tmp/claud[e]|\.codex/auth\.json" \
  $D --exclude-dir=target
grep -rn -E 'env::var|\.codex|\.claude|CONDUIT_' $D/src
grep -rn -E "[[:space:]]+$" $D --exclude-dir=target
```

`target/` stays ignored and unstaged. Report whether the launch test ran, each choice this order
leaves to you, and anything the plan or agent-codex leaves open, as your own choices and
findings. Do not stage anything.

## Record

Run on 2026-11-12 on the Mac, from base c60825d: the worker (1,371 s) and a fresh verifier (450 s),
then a second worker round (392 s) and a fresh verifier (447 s). Round one's three musts were fixed
in round two: a trace could take a barrier file's name; option values equal to `resume` were read
as the subcommand; and unknown fields went unnamed. Round two's verifier left three musts, which
[order 148](148-scr1-finish.md) finished: a trace name in another case still collided with a
barrier file on a case-insensitive file system; short redaction entries could corrupt an
invalid-script report; and markers inside numbers escaped redaction, while a second pass could
break the summary's JSON.

The result: `experiments/015-s4-partial-setup/scripted-client/`, the crate `scr-client`, S4's
scripted client, launched as an Agent through agent-codex's `--client-program`, authenticating
only through the wrapper's header helper, sending keyed, fresh and unkeyed calls exactly once each
through rmcp's raw request path, holding barriers by an acknowledged file handshake, and keeping
redacted traces and a summary that completes the turn. Its tests ran on the Mac, the wrapper
launch test included.

The worker's findings: `script-deadline` was missing from the summary's list of causes and was
added; the summary's call label is `call-INDEX`; an isolated run's `https` engine address needs
TLS that this crate does not build, for the order that first needs it; and `tools/set_names.py`
does not reach `experiments/`, so the names this crate copies from `spec/` need a manual update
when they change. Its own choices include rmcp's automatic lifecycle (current discovery, with the
older initialization as a fallback), literal loopback addresses only, symbolic-link barrier
directories refused, and a shortened deadline injected only in a unit test.
