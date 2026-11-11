# Experiment 009 harness

Historical method as run on 2026-11-09. The [harness code at C0](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/009-native-launch)
is superseded by [the engine](../../engine/README.md) and the
Agent packages for [Claude Code](../../packages/agent-claude-code/README.md) and
[Codex](../../packages/agent-codex/README.md). The commands below describe that archived harness.

This implements the [pre-registration](README.md) under
[work order 010](../../docs/implementation/WORK_ORDERS.md#order-010). It runs a stub, not an
engine. No real client was run by the worker. Offline fixtures are not experiment results.

## Commands for the admin

From this directory, build all the programs, then inspect the plan:

```sh
cargo build --release --bins
cargo run --release -- --dry-run --output /tmp/conduit-009-evidence-2026-11-09
```

The admin runs the clients with:

```sh
target/release/native-launch-harness --output /tmp/conduit-009-evidence-2026-11-09
```

Every invocation requires an explicit `--output`, including a dry run. Choose a fresh absolute
path outside this repository. Existing directories, paths containing `..`, and symlink aliases
into the repository are refused. A dry run creates nothing, binds no socket, starts no process
and prints every scenario, identity subject, wrapper command, client arguments, environment,
configuration and prompt. Its port and resumed thread ID are placeholders.

There are nine scenarios: `claude-r1` to `claude-r3`, `codex-r1` to `codex-r3`, and `pair-r1`
to `pair-r3`. Each pair runs one Agent of each client simultaneously, against one stub, with
two distinct credentials. That is twelve Agent sessions and twenty-four turns. The order's
three repetitions are applied to the paired scenario too. Budget twelve minutes, an estimate;
`--timeout-secs` defaults to 180 per client turn, so eighteen sequential turn boundaries could
take about 54 minutes at that deadline, plus startup and cleanup.

Needs: Rust 1.97.1 through rustup, a C compiler for `ring`, crates available locally or network
access for Cargo, and installed Claude Code and Codex on `PATH`. The reference versions are
Claude Code 2.1.318 and Codex 0.167.0 from experiment 008. Each client needs its existing owner
subscription login available in the login session in which the admin runs the harness. Versions
and version-command status are recorded with the same filtered environment. The worker used
only CLI help and local documentation, and used network access only for crates. No owner
configuration or login file is inspected, copied or changed by the harness. Separate client
configuration homes and separate logins remain a step requiring the owner, not implemented here.

## The wrappers

`claude-wrapper` and `codex-wrapper` are separate binaries, sharing [src/wrapper.rs](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/009-native-launch/src/wrapper.rs) and
[src/client.rs](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/009-native-launch/src/client.rs). They speak newline-delimited MCP over stdin/stdout using the official Rust
SDK, `rmcp` 3.9.0. They expose exactly one tool, `turn`, with one required string argument,
`message`, and reject unknown tools, extra arguments and non-string messages before launching
a client. Its result has text content plus `structuredContent` containing `text`, the client's
JSON `events`, and `start_report`. A client failure returns `isError: true` and poisons that
session: another call cannot silently start a replacement conversation. The experimental tool
schema is local to this harness; it is not a proposed Agent type contract.

Claude starts on the first tool call, retains one process and its open stdin, and receives
one stream-JSON user message per call. A second message is sent only after the first `result`
event. Codex starts `exec` for the first call and `exec resume` with the exact reported thread
ID for each later call. Every process must report that same ID and complete successfully;
`--last` and `--ephemeral` are never used. Calls to a single wrapper are serialized. Pair
requests are polled concurrently. EOF, client errors, nonzero Codex exits and missing or changed
thread IDs are failures. Each turn has a deadline, partial events are flushed as they arrive,
and cleanup closes MCP, then kills the wrapper's private process group if necessary.

The runner scripts MCP initialization at revision `2025-11-25` and verifies the one-tool
catalog before asking for a turn. The wrappers use the SDK's supported revisions. The shared
HTTP stub retains experiment 008's revisions `2025-06-18`, `2025-11-25` and `2026-07-28`;
its logs record what each native client negotiates. The wrappers' MCP server role and their
native clients' consumer role are separate connections.

## Launch and identity

Every wrapper starts with an environment cleared first, then populated with only `HOME`,
`LOGNAME`, `PATH`, `SHELL`, `TERM` and `USER` where present and not beginning with `()`, plus
the two variables from the assigned names in `spec/names.yaml`: `CONDUIT_URL` and
`CONDUIT_IDENTITY_DIR`. The runner reads only those six ambient names. It does not read ambient
API keys to filter them. The wrapper applies the same filtering to its client, explicitly
removing `ANTHROPIC_API_KEY` and `OPENAI_API_KEY` as well. No cache, temporary-directory,
proxy, configuration-home or parent-session variable is added.

The working directory is a newly created empty directory for that Agent; configurations and
evidence live beside it, not inside it. Output and Agent directories are private (0700), and
token files are 0600. DL16's choices here are macOS/Unix, the current operating-system user and
login session, and no extra environment-variable list. These are attribution experiments,
not process isolation: the shared operating-system user can read the other Agent's credential.

Experiment 008 is a path dependency. Its small library extension, `start_agents`, accepts
identity subjects and their directory paths, checks unique subjects, and mints separate
ES256 JWT-SVIDs. `RunningStub::change_access` performs the existing notification machinery's
state change from the runner. In this mode it closes `files` and opens `notes` together.
Admission verifies each request independently and logs its instance ID, never its bearer token.
All minted credentials are redacted from request bodies and wire data. Existing experiment 008
entry points and cases retain their behavior and synthetic audience. Experiment 009's audience
is the actual loopback engine URL, including scheme, host, port and path.

Each identity directory gets the assigned token, JWK set, engine ID and an empty `ca.crt`.
This is PRV-022's HTTP-loopback case: no TLS certificate is configured or claimed, and the
empty certificate bundle checks nothing. Each client's header helper invokes its wrapper's
`--headers-helper` mode and reads the token on every invocation. Claude's helper locates the
directory through `CONDUIT_IDENTITY_DIR`. Codex filters its helper's environment and rejects
`env_vars` for HTTP servers, so its helper command carries `--identity-dir` with the directory
path. That argument takes precedence over the variable.
The helper prints the header map only to the client and records a timestamp without the token.
No token is supplied as a command argument or variable.

## Client flags and settings

Claude uses experiment 008's `--setting-sources project`, `--strict-mcp-config`, a single
`--mcp-config`, `--tools ""`, and `--allowedTools 'mcp__stub__*'`, plus its settings file,
`--disable-slash-commands`, `--no-session-persistence`, streamed input/output, and verbose
JSON events. Settings disable hooks and supply an empty plugin map. In addition:

| Purpose | Exact setting or flag | Confirmation available to the worker |
| --- | --- | --- |
| Deny messaging, Agent listing and remote triggering | `permissions.deny: ["SendMessage", "ListAgents", "RemoteTrigger"]`, also `--disallowedTools SendMessage,ListAgents,RemoteTrigger` | Local settings reference documents `permissions.deny`; installed `claude --help` documents the flag |
| Turn off claude.ai connectors | `disableClaudeAiConnectors: true` | Local settings reference says it prevents fetching and connecting, and leaves explicit `--mcp-config` servers available |
| Refuse other sessions' messages | `crossSessionInbound: "refuse"` | Local settings reference says `refuse` drops incoming messages |

All three setting names and values were confirmed; no requested setting was left guessed.
The saved documentation consulted was `scratchpad/dl1/settings-reference.md` on the admin's
machine, sections `permissions.deny`, `disableClaudeAiConnectors` and `crossSessionInbound`.
The corresponding public reference is
[Claude Code settings](https://code.claude.com/docs/en/settings-reference).
That URL identifies the source; the worker did not fetch it. CLI help warns that print mode
can silently ignore invalid settings and that managed settings still apply. Flags and generated
settings therefore establish the requested setup, not evidence that the native client obeyed it.

Codex uses `--ignore-user-config --strict-config --ignore-rules --disable apps`, a read-only
sandbox, disabled web search, only the stub's MCP configuration, its `http_headers_helper`,
`startup_readiness="connection"`, `required=true` and
`default_tools_approval_mode="approve"`. Exec-only `--sandbox` and `--color` precede `resume`;
the same remaining isolation overrides follow it, as in experiment 008. Codex retains its
built-in tools under its read-only sandbox. Prompts restrict use to the stub; other tool use
must be classified from observations. Authentication is never inferred from these flags.

## Turns and records

Turn one requests one `files.read.v1` of `hello.txt`. Once all first-turn results are back,
the runner marks turn two in the stub log, closes `files`, opens `notes`, and notifies existing
streams before sending any second-turn request. Turn two asks for the newly opened notes read
and an old files read if available, otherwise an unavailability report. Neither prompt asks the
model to advance the stub. Prompts are fixed before execution, not constructed from model text.

Each scenario has `stub.jsonl` and `status.json`; pair scenarios share that one stub log. Each
Agent subdirectory holds:

| Artifact | Meaning |
| --- | --- |
| `launch.json`, `version*.txt`, `version-status.json` | Requested wrapper launch, exact environment, working directory, instance and installed version |
| `exchange.jsonl`, `wrapper-stderr.txt` | Every scripted MCP request/response and notifications, plus wrapper diagnostics |
| `mcp.json`, `settings.json` | Explicit native-client configuration |
| `turn-N-plan.json`, `process-N-stderr.txt` | Each actual native-client process launch; Claude turn two shares its first process and plan |
| `turn-N-prompt.txt`, Claude's `turn-N-input.jsonl` | Fixed prompt and exact streamed input sent |
| `turn-N-events.jsonl`, `turn-N-final.txt` | Raw client events and reported final text; partial records survive errors |
| `turn-N-start-report.json` | Startup-reported tools, MCP servers and authentication, with source line and JSON pointer; missing fields explicitly say `not_reported` |
| `turn-N-status.json`, `turn-N-response.json` | Native-client completion status and successful wrapper response; failures are retained in the exchange and scenario status |
| Codex's `session.json` | Exact ID reported by `thread.started` |
| `identity/`, `identity/helper.jsonl`, `work/` | Throwaway identity files, helper-read timestamps and the private working directory |

Start reports extract only initialization/startup/authentication events, not model prose or tool
results. In particular Codex may report neither its tool catalog nor its authentication in its
public JSON stream; absence is `not_reported`, never proof of an empty catalog or a subscription
login. Raw events and stderr remain available for classification. An unsent turn has no invented
client events; the scenario status identifies the failure that prevented it. Nothing here
asserts exact model-input visibility or that the owner's configuration and login reach were
successfully excluded. Those are the admin's real-run observations still to make.

## Offline checks

```sh
cargo test --release
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo run --release -- --dry-run --output /tmp/conduit-009-plan-only
```

The `scripted-client` binary is an offline fixture, not an Agent. Tests drive each wrapper's
MCP interface with it, exercise both turns and all nine runner scenarios, verify same-process
Claude and exact-session Codex continuation, compare actual client environments, and check
invalid calls, EOF, errors, timeouts, failed exits and changed session IDs. Scripted HTTP tests
check both credentials, correlate every request's distinct JSON-RPC ID with its expected
subject, and verify invalid credentials establish no instance. Both throwaway tokens appear in
a JSON-RPC string ID echoed by the stub: request and wire logs must each contain redaction
markers and neither token. Header-helper tests execute each configured command with quoted
paths, including Codex with no identity variable and with a conflicting value. Tests also check
file modes and the combined access change. Dry-run tests check every scenario,
required explicit output, path validation and absence of output creation. Experiment 008's
existing tests are also run because its library changed.
