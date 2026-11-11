# agent-claude-code

Maintainer: the Conduit project's owner. Native-client launch baseline: Claude Code 2.1.318, from
experiment 009. Work orders 023 and 024 tested this package with its scripted client only; the real
client closeout is still required. No client version command or model was run for this order.

This standalone Rust crate has no dependency on another repository directory. Its lockfile
and pinned toolchain travel with it. Its manifest exports two Agent Types, a conversation
capability and the explicit `turn.v1` operation. The plain MCP wrapper exposes only `turn`;
the manifest maps the versioned operation to it. In `.v1`, both reach inputs are required booleans.
Authors may omit `model`: the engine fills its empty-string default, leaving the native
client's default in force. MAN-017 requires the bound scalar to be required in the resolved
configuration, so the Type's required list includes model. This is the logged nearest sound
reading of work order 023's optional model input.
Setup initializes the wrapper and starts no native client or model work. Inputs and outputs
are excluded from Conduit records by the Type.

Run these commands in this directory:

```sh
cargo test --release
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo run --release --bin package
```

The last command builds the host wrapper and writes `dist/agent-claude-code.tar.gz` and its `.sha256`
sidecar. It prints an absolute `ARCHIVE#SHA256` argument for `conduit install --store DIR ARCHIVE#SHA256`, while `conduit serve` holds that store.
Install required dependencies first, then use `conduit submit --store DIR PROJECT_DIR`.
The gzip header and USTAR member metadata are deterministic for identical binary/YAML bytes:
sorted names, zero owner/group/time, binary mode 0755, YAML mode 0644. This is a host archive,
not a portable executable for another OS/architecture. Cargo builds and distribution output
are ignored. No script or installed native client is needed to pack.

The runner supplies a private empty working directory, `CONDUIT_URL` and
`CONDUIT_IDENTITY_DIR`. For `.v1`, the manifest binds all three Type inputs through JSON Pointers to
`AGENT_ACCOUNT_SERVICES`, `AGENT_PEER_MESSAGING` and `AGENT_MODEL`. The wrapper inherits only
HOME, LOGNAME, PATH, SHELL, TERM and USER, plus the run's two identity variables, into its
client. It removes ANTHROPIC_API_KEY and OPENAI_API_KEY. The owner's subscription login is
shared; a separate login home is future work. Only the helper mode reads `token`, printing
one JSON Authorization header and nothing else. The token is never placed in environment
variables, arguments or logs. The wrapper retains no prompt, raw event stream or stderr log.
A failed turn poisons the conversation; no automatic restart or repeated model work occurs.
The per-turn deadline is 180 seconds for `.v1`, and `turnDeadlineSeconds` for `.v2`,
at most 604800 seconds; after it the client ends and the turn fails.
Cancellation while queued skips the turn. Active cancellation ends the client and poisons
the conversation, preventing an implicit restart or repeat.
`--client-program PATH` is an explicit scripted-test injection, absent from the package item.

A successful result contains `text` and `clientTools`. Startup observations retain their
positions, raw `reportedNames`, filtered engine `names`, and freshness. `reported` means
the first turn's startup list establishes a current catalog. On later turns, session/thread
startup lists are `unavailable` with `session_init_unverified` freshness: they may repeat
historical tools even when names differ. Missing lists are `not_reported`. Neither status
establishes a current catalog, and neither is replaced with engine lists or model prose.
Experiment 009's repeated Claude init is a regression fixture. Notifications start no extra
turn. Closeout compares current client evidence with the engine's separate private list.

In `.v1`, Claude Code runs one streamed process for the wrapper's lifetime. Settings and MCP files are
written in the run's directory. `--setting-sources ""` excludes all setting sources,
including project discovery of ancestor CLAUDE.md files (finding 3-04). The engine server
and its header helper come solely from explicit `--mcp-config` with `--strict-mcp-config`;
the engine allow rule comes solely from explicit `--settings`. Hooks/plugins are disabled.
`--tools ""` removes built-in tools that reach the machine, while `mcp__engine__*` is allowed
once. With accountServices false, claude.ai connectors are disabled. With peerMessaging false,
SendMessage, ListAgents and RemoteTrigger are denied and crossSessionInbound is refuse. True
selects and allows only those three peer tools beside engine tools and omits the inbound
refusal, leaving the native client's inbound default. Machine tools remain unselected. Sentinels and a real incoming
message must still demonstrate the flags/settings' actual behavior in closeout.

For `.v1` and `.v2` with no directory, the client's own tools that reach the machine, a shell, code execution, a browser, another
Agent, hooks and plugins are off by default, alongside account services and messaging.
With both reach inputs false, no built-in machine or peer tool is selected; engine MCP tools
remain. Account connectors and the three named peer tools are explicit opt-in routes around
the engine. Closeout must record any other native tool the client still exposes and test its
reach. Shared login and OS-user access to files and run credentials remain mediated limits;
these launch settings do not provide OS isolation.

## The .v2 Agent

The package also exports claude-code.v2, implemented by the separate `wrapper-v2` item.
All properties are required after SCH-009 fills their defaults; authors may omit them.
A null `directory` gives no environment variable (PRV-007).

| Input | Default / bounds | Wrapper variable |
| --- | --- | --- |
| `directory` | `null`; otherwise an absolute path to an existing directory | `AGENT_DIRECTORY` |
| `turnDeadlineSeconds` | `180`; integer from 1 through 604800 | `AGENT_TURN_DEADLINE_SECONDS` |
| `effort` | `''`; otherwise lowercase ASCII letters | `AGENT_EFFORT` |
| `accountServices` | `false` | `AGENT_ACCOUNT_SERVICES` |
| `model` | `''` | `AGENT_MODEL` |
| `peerMessaging` | `false` | `AGENT_PEER_MESSAGING` |

Absent or empty effort/model leaves the client's own default. The presence of
`AGENT_TURN_DEADLINE_SECONDS` selects `.v2`; its absence selects `.v1`.
A null directory selects `.v1`'s tools and sandbox. Naming a directory grants the
client's own tools; there is no separate switch. Limits on this reach, such as allowed
hosts or a read-only directory, will come from a confining runtime provider chosen
with `runtime:` (ruling 75, B2), rather than Agent inputs.

When the template names a directory, nothing confines the client in v0. Its shell, code
execution, file edits and network run as the engine's operating-system user, with the
client's sandbox and approval prompts off. It can reach every file, process, network address, credential and
signed-in account that user can reach: the engine's store and root's client token;
`CONDUIT_IDENTITY_DIR`, with this run's token; and the owner's client login, which every
Agent shares until order 050 gives each its own home. It can push through the keychain,
whatever HOME is, and reach other git worktrees that share one `.git`. This reach does
not pass through Conduit: no Conduit grants, limits or records it (ruling 70).

The shell can reach services, the user's browser and desktop even with
`accountServices: false`; that input shapes only the client's tool list. The mediated
access paragraph above holds only for `.v1` and for `.v2` with no directory.

The wrapper checks that `directory` is absolute and exists, then canonicalizes it.
The client starts there, or in the run's private directory when it is null.
`Config::cwd` remains the run's directory. Work orders explicitly tell workers to read
AGENTS.md; automatic project instruction discovery stays off.

Claude Code keeps `.v1`'s arguments and their order. With own tools off it appends
only optional `--effort E`. With own tools on, `--tools` explicitly selects
`Bash,Read,Edit,Write,Glob,Grep,WebFetch`, appending `,WebSearch` with accountServices
and then `,SendMessage,ListAgents,RemoteTrigger` with peerMessaging. It never selects
`default` or omits `--tools`. `--allowedTools`, `--disallowedTools` and all other
arguments remain as in `.v1`. It appends
`--permission-mode bypassPermissions --permission-prompts none`, then optional
`--effort E`. A prompt is denied instead of waiting for an answer the wrapper cannot give.

`--setting-sources ""` stays. Settings change only permissions.defaultMode to
`bypassPermissions` with own tools on, keeping `dontAsk` otherwise. Hooks, plugins,
sub-agent and scheduling tools remain off; connectors follow accountServices.
`prepare()` writes settings.json and mcp.json in the run's private directory, never
in the named working directory, so a working-tree commit cannot pick them up.
The engine entry in mcp.json gets `timeout=N*1000`, and the client's environment gets
`MCP_TOOL_TIMEOUT=N*1000`; N is turnDeadlineSeconds. Both multiplications use checked
arithmetic. This is the only client environment addition; `.v1`'s environment and
mcp.json stay unchanged.


`conduit serve --call-timeout-seconds` must be longer than `turnDeadlineSeconds`.
With their defaults, 120 and 180, the engine reports a turn longer than 120 seconds
before the wrapper's deadline. Longer turns may also require the engine fix in order 074.

## Turn boundaries and failures (both versions)

An `error` event alone does not end a turn. The wrapper keeps only the class of its
latest message in that turn. Codex succeeds at `turn.completed` followed by successful
process exit; `turn.failed` fails with its own `error.message` class. Claude Code ends
at `result`, failing when `is_error` is true with the class of `result`. A retried stream
error followed by a successful boundary succeeds.

A failed call has `isError: true` and text
`client turn K failed: CAUSE; session cannot restart implicitly`.
Later calls return `session failed at turn K: CAUSE; implicit restart refused`.
K counts turns from 1 and continues across resumes. Active cancellation records
`cancelled`; queued cancellation starts no turn. There is no implicit restart.

The fixed causes and their paths are:

| Cause | Path |
| --- | --- |
| `deadline: no turn boundary within N seconds` | Turn deadline, including any exit wait; `.v2` appends ` (input turnDeadlineSeconds)` |
| `cancelled` | Active cancellation |
| `client failure: CLASS` | Fatal boundary, or latest error event before EOF/pipe failure |
| `client exit: status N` / `client exit: signal N` | Nonzero exit before the boundary, or unsuccessful Codex exit after completion |
| `client exit: status unknown` | Failed exit wait, unless the kept error class takes precedence |
| `EOF before turn boundary` | EOF/pipe failure with successful exit, or no exit within the bounded wait |
| `no thread` | Codex completed without reporting a thread |
| `invalid client output` | Non-JSON line or `thread.started` without a thread ID |
| `resume changed thread` | Codex resume reports a different thread |
| `client did not start` | Spawn failure or missing stdout pipe |

At EOF before the boundary, missing stdin, message write/flush/close failure or a line
read failure, `EXIT_WAIT` allows at most two seconds for exit without killing the client.
This counts toward the turn deadline. The latest error class takes precedence, then
nonzero status/signal, then EOF. Cleanup ends the client after the cause is selected.

CLASS uses one case-insensitive, ordered substring table; the first matching class wins:

| Class | Substrings |
| --- | --- |
| `usage-limit` | `usage limit`, `usage_limit`, `quota` |
| `rate-limit` | `rate limit`, `rate_limit`, `too many requests`, `429` |
| `authentication` | `authentication`, `unauthorized`, `401`, `login`, `log in`, `api key`, `credential` |
| `context-window` | `context window`, `context_window`, `context length`, `context_length`, `prompt is too long` |
| `other` | No match or no text |

These substrings are an initial list tested with synthetic failures, not real failures.
Failure results, errors and stderr never include raw client text. Invalid configuration
still initializes MCP and lists `turn`; every turn reports `invalid configuration: ...`,
naming the input/variable, reason and directory path where relevant, without starting a
client or poisoning a session.
