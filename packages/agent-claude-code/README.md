# agent-claude-code

Maintainer: the Conduit project's owner. Lowest tested client: Claude Code 2.1.318. Work orders
023 and 024 tested this package with its scripted client only. Experiment 027 used Claude Code
2.1.318. The version checks added in 0.2.5 use the scripted client in tests.

This standalone Rust crate has no dependency on another repository directory. Its lockfile
and pinned toolchain travel with it. Its manifest exports five Agent Types, an account home
Resource Type, and a conversation capability with two explicit operations, `turn.v1` and
`new-conversation.v1`. The plain MCP wrapper
exposes `turn` and `new-conversation`. The `.v2` item also exposes the hook tools `retire`, `account`,
`sign-out` and `sign-in`. The manifest maps each versioned operation or hook to its tool. See the
[account and sign-out hooks](#the-v2-account-and-sign-out-hooks) and the
[sign-in hook](#the-v2-sign-in-hook), which `claude-code.v2` requires with its `signIn`.
In `.v1`, both reach inputs are required booleans.
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

Built offline on the same host with the same toolchain, the archive has the same digest on
every run.

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
client. It removes ANTHROPIC_API_KEY and OPENAI_API_KEY. A `.v1` client uses the default
home of the engine's user, and with it that user's subscription login, shared by every
`.v1` Agent; each `.v2` Agent has its own home ([below](#the-v2-agents-own-home)).
Only the helper mode reads `token`, printing
one JSON Authorization header and nothing else. The token is never placed in environment
variables, arguments or logs. The wrapper retains no prompt, raw event stream or stderr log.
A failed turn poisons the conversation; no automatic restart or repeated model work occurs.
The per-turn deadline is 180 seconds for `.v1`, and `turnDeadlineSeconds` for `.v2`,
at most 604800 seconds; after it the client ends, for `.v2` with its
[whole process tree](#the-v2-clients-process-tree), and the turn fails.
Cancellation while queued skips the turn. Active cancellation ends the client, for `.v2` with
its [whole process tree](#the-v2-clients-process-tree), and poisons the conversation, preventing
an implicit restart or repeat.
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
reach. For `.v1`, the shared login, and for both, OS-user access to files and run
credentials remain mediated limits; these launch settings do not provide OS isolation.

## The .v2 Agent

The package also exports claude-code.v2, implemented by the separate `wrapper-v2` item.
All properties are required in the resolved instance configuration after SCH-009 fills their defaults.
A null `directory` gives no environment variable (PRV-007).

| Input | Default / bounds | Wrapper variable |
| --- | --- | --- |
| `directory` | `null`; otherwise an absolute path to an existing directory | `AGENT_DIRECTORY` |
| `turnDeadlineSeconds` | `180`; integer from 1 through 604800 | `AGENT_TURN_DEADLINE_SECONDS` |
| `effort` | `''`; otherwise lowercase ASCII letters | `AGENT_EFFORT` |
| `accountServices` | `false` | `AGENT_ACCOUNT_SERVICES` |
| `model` | `''` | `AGENT_MODEL` |
| `peerMessaging` | `false` | `AGENT_PEER_MESSAGING` |
| `account` | none; a string or null | none |

Absent or empty effort/model leaves the client's own default. The presence of
`AGENT_TURN_DEADLINE_SECONDS` selects `.v2`; its absence selects `.v1`.
A null directory selects `.v1`'s tools and sandbox. Naming a directory grants the
client's own tools; there is no separate switch. Limits on this reach, such as allowed
hosts or a read-only directory, will come from a confining runtime provider chosen
with `runtime:` (ruling 75, B2), rather than Agent inputs.

`account` names the subscription account that the client is to be signed in to.
Each resolved `.v2` instance configuration must contain `account`, which has no Type default.
Its value can come from the template or an instantiation argument.
A null `account` signs in to no subscription.
The package's 0.2.0 is subscription-only: the Type states no `apiKey` and takes no API key, and API access comes later as a declared route (ruling 83).
Nothing falls back from a sign-in to a key (ruling 82, point 1).
No item of this package binds `account`, so it reaches neither the wrapper nor the client.

When `directory` is a string, the engine reads it through the Type's `toolsDirectory: {directory: /directory}` declaration (CTR-014) only to report the reach of the client's own tools on inspection.
The item's `AGENT_DIRECTORY` binding passes the string value to the wrapper.
The Type's description says that these tools run as the engine's operating-system user on the engine's own runner, and, under another runtime provider, on the machine of the instance's node with the limits that provider's contract states.

On the engine's own runner, the client has unconfined reach in v0 when the configuration
names a directory. Its shell, code execution, file edits and network run as the engine's
operating-system user, with the client's sandbox and approval prompts off. It can reach
every file, process, network address, credential and signed-in account that user can reach:
the engine's store and root's client token;
`CONDUIT_IDENTITY_DIR`, with this run's token; the default home of the engine's user, with
its login, and every other Agent's own home in the engine's store. It can push through the keychain,
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
arithmetic. With `CLAUDE_CONFIG_DIR`, these are the only client environment additions;
`.v1`'s environment and mcp.json stay unchanged.


`conduit serve --call-timeout-seconds` must be longer than `turnDeadlineSeconds`.
With their defaults, 120 and 180, the engine reports a turn longer than 120 seconds
before the wrapper's deadline. Longer turns may also require the engine fix in order 074.

## The .v2 Agent's own home

The `.v2` wrapper requires the state directory that the engine's runner gives each
instance (PRV-029). It reads `CONDUIT_STATE_DIR` from its own environment alone. No input
sets the variable or the home. The wrapper creates `client-home` in that directory
with mode 0700 on Unix when it is missing. At every launch it checks the home and sets
`CLAUDE_CONFIG_DIR` to its canonical path. `HOME` still comes from the runner.

The wrapper reports these refusals.

- `client home (CLAUDE_CONFIG_DIR): CONDUIT_STATE_DIR is unset`
- `client home (CLAUDE_CONFIG_DIR): CONDUIT_STATE_DIR is not an absolute path`
- `client home (CLAUDE_CONFIG_DIR): CONDUIT_STATE_DIR is not an existing directory`
- `client home (CLAUDE_CONFIG_DIR): cannot create the home in CONDUIT_STATE_DIR`
- `client home (CLAUDE_CONFIG_DIR): the home resolves outside CONDUIT_STATE_DIR`
- `client home (CLAUDE_CONFIG_DIR): the home is not a directory`

If a runner supplies no state directory, every `.v2` turn fails with the unset diagnostic.
A `.v1` launch works as in 0.1.1 and uses the engine user's default client home.
The engine's own runner removes the home with the instance (PRV-029, LIF-035).
Before that, the [retire hook](#the-v2-retire-hook) signs Claude Code out and empties the home.
Under another runtime provider, that provider's contract states what happens to the home.
The login in the home belongs to Claude Code, which keeps it where it stores logins for
that directory.

The wrapper does not refuse a home that holds a `.env`, since no source the package
cites shows Claude Code reading one from that directory.

## The .v2 client's process tree

Each `.v2` launch starts Claude Code as the leader of a new process group. On a failed turn,
a deadline, a cancellation, the wrapper's TERM or the end of its input, the wrapper finds
live processes in the groups whose leaders' start times it recorded and follows parent
links from them.
It sends TERM, sends KILL to processes still live after two seconds, and waits up to one
second more. It reads each process's PID, parent PID, process group, start time and zombie
state from `/proc/PID/stat` on Linux and `proc_pidinfo` on macOS. This read accesses no
process's environment or arguments.

The wrapper answers a failed turn after the stop and names each PID that outlived it in the
failed call's [fields](#the-v2-failed-turns-fields). On TERM the wrapper writes the note
`client tree: PIDs P1, P2 outlived the stop`, or
`client tree: the process table could not be read, so the stop is unconfirmed`, to its
stderr and exits with status 143.
The engine's own runner discards the wrapper's stderr. At the end of input the wrapper also
writes any stop note to stderr. A successful turn stops nothing, so anything a turn's tools
leave running stays until the next stop unless it exits on its own. The `.v1` launch and stop
are those of 0.1.1.

Limits:

- The wrapper samples the tree only during a stop. A process that left the client's group and
  lost its parent links to the tree before the stop's first read is neither stopped nor named.
  A double fork that also starts a new session can do this. If Claude Code exits before the stop,
  as at an exit or EOF failure, a child in a session of its own is neither stopped nor named.
- The client's group lies outside the run's process group, the unit the engine's own runner
  uses to stop the run ([PRV-030](../../spec/promises/providers.yaml), [DL16](../../engine/CONFORMANCE.md)).
  If the wrapper is killed without warning, at the engine's loss or by the runner's KILL after
  a `stop_grace` shorter than three seconds, the client's tree is neither stopped nor seen.
  With another runtime provider, its contract states what reaches the client's group.
- When the process table cannot be read, including on systems other than Linux and macOS, the
  stop signals only groups whose leader's start time it can confirm and reports that the stop
  is unconfirmed. If the wrapper could not record a group leader's start time at launch,
  the stop uses parent links from other tracked processes to find members of that group
  and reports the stop as unconfirmed if the group still has live members at the end.

## Turn boundaries and failures (both versions)

An `error` event alone does not end a turn. The wrapper keeps only the class of its
latest message in that turn. Codex succeeds at `turn.completed` followed by successful
process exit; `turn.failed` fails with its own `error.message` class. Claude Code ends
at `result`, failing when `is_error` is true with the class of `result`. A retried stream
error followed by a successful boundary succeeds.

A failed call has `isError: true` and text
`client turn K failed: CAUSE; session cannot restart implicitly`.
A `.v2` failed call also holds the [failed turn's fields](#the-v2-failed-turns-fields).
Later calls return `session failed at turn K: CAUSE; implicit restart refused`.
K counts turns from 1 and continues across resumes. A
[new conversation](#a-new-conversation-both-versions) counts from 1 again.
Active cancellation records `cancelled`; queued cancellation starts no turn. There is no implicit restart.

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
nonzero status/signal, then EOF. Cleanup ends the client after the cause is selected; for
`.v2` it first stops the client's whole process tree.

For text failures, CLASS uses this case-insensitive table. The first matching class wins.

| Class | Substrings |
| --- | --- |
| `usage-limit` | `usage limit`, `usage_limit`, `quota`, `hit your session limit`, `hit your weekly limit`, `hit your opus limit`, `hit your sonnet limit` |
| `rate-limit` | `rate limit`, `rate_limit`, `too many requests`, `429` |
| `authentication` | `authentication`, `unauthorized`, `401`, `login`, `log in`, `api key`, `credential` |
| `context-window` | `context window`, `context_window`, `context length`, `context_length`, `prompt is too long` |
| `network` | `network`, `connection`, `disconnected`, `error sending request`, `timed out`, `timeout`, `dns`, `getaddrinfo`, `econnreset`, `econnrefused`, `enotfound`, `unreachable`, `fetch failed` |
| `structured-output` | Result subtype `error_max_structured_output_retries` |
| `other` | No match or no text |

These substrings are an initial list tested with synthetic failures, not real failures.
Failure results, errors and stderr never include raw client text. Invalid configuration
still initializes MCP and lists `turn`; every turn reports `invalid configuration: ...`,
naming the input/variable, reason and directory path where relevant, without starting a
client or poisoning a session.

## The .v2 failed turn's fields

When a `.v2` or later turn fails, the result's `content` holds the failure text followed by a
second text block containing one JSON object, `{"failedTurn":{...}}`. The `failedTurn`
object has these fields.

| Field | Value |
| --- | --- |
| `turn` | The turn number, counted from 1 across resumes |
| `cause` | The fixed kind selected from the cause text below |
| `class` | The class from `client failure: CLASS`, or `null` for every other cause |
| `clientStarted` | Whether the session held a client when the failure was recorded, before stopping it |
| `survivors` | The PIDs that outlived the stop in ascending order, or an empty list |
| `stopConfirmed` | `true` if the stop was confirmed, otherwise `false` |
| `limit` | For `usage-limit`, the window and reset report described [below](#a-usage-limits-window-and-reset). Otherwise `null`. |

| Cause | Cause text |
| --- | --- |
| `deadline` | Begins with `deadline: ` |
| `cancelled` | `cancelled` |
| `client-failure` | Begins with `client failure: ` |
| `client-exit` | Begins with `client exit: ` |
| `eof` | `EOF before turn boundary` |
| `no-thread` | `no thread` |
| `invalid-client-output` | `invalid client output` |
| `resume-changed-thread` | `resume changed thread` |
| `client-did-not-start` | `client did not start` |
| `client-home` | Begins with `client home (` |
| `unclassified` | Any other cause text |

The fields carry fixed words and numbers, with no client text. The fields are not outputs
of `turn.v1`, whose schemas stay as 0.1.1 published them. The failed result omits
`structuredContent`. The engine gives the plain implementation's `content` to the caller
unchanged after its own text block. [IFC-025](../../spec/interface/implementation.yaml)

The classes are listed in this README and in no schema or description. A `.v1` turn
failure, an invalid configuration response and the refusal of a later call each carry one
text block.

The turn items are plain. Under PND-005 they report a missing or lapsed login only by a
URL request. A turn has no login URL to give and sends no request. If a login lapses
during a turn, the turn fails with class `authentication` when the client's text matches
that class. The engine ends the call unknown under IFC-010 and raises no sign-in action
from the turn. The instance's next account check detects the lapse.
[PND-005 and PND-011](../../spec/promises/pending.yaml),
[IFC-010 and IFC-016](../../spec/interface/implementation.yaml)

## The .v2 retire hook

`claude-code.v2` lists `retire-hook.v1` under `requires`. Its default is `wrapper-v2`, the
item that serves `turn.v1`, so one run serves both (TPL-004, TPL-019). The item serves
the hook as the tool `retire`, which takes no arguments. The `.v1` item lists no such tool.
At retirement, the hook applies to an instance for which a setup was ever dispatched,
including a setup with an unknown outcome (TPL-018, LIF-025). When the hook is due, the
engine dispatches it before stopping the run. If the runtime provider reports the run
`stopped`, the engine first asks for the run. If the provider reports `unknown`, the engine
leaves the hook undispatched and reports its part unavailable (TPL-018).

1. The hook stops the client's process tree as a failed turn does. From then on, it
   refuses every turn with `session retired; implicit restart refused`. If a process
   outlives the stop or the stop is unconfirmed, the hook fails with the stop's note.
2. It checks the home as every launch does, with the same refusals, and creates the
   home if it is missing.
3. It runs `claude auth logout` with `CLAUDE_CONFIG_DIR` set to the Agent's home and the launch's
   environment. It discards the client's output and waits up to 60 seconds for exit.
4. On exit status 0, it checks the home again as in step 2, and fails with
   `client home (CLAUDE_CONFIG_DIR): cannot empty the home` if the home now resolves to another
   directory, removing nothing. It then removes every entry in the home. For a link it removes the
   link itself. It keeps the home directory and answers `client signed out and its home emptied`.

The hook reports failures with these fixed texts.

- `client logout did not start`
- `client logout failed: client exit: status N`
- `client logout failed: client exit: signal N`
- `client logout failed: client exit: status unknown`
- `client logout failed: no exit within 60 seconds`. At the deadline the hook stops the logout's
  tree and appends the stop's note if there is one.
- `client home (CLAUDE_CONFIG_DIR): cannot empty the home`

If the state directory is unset or relative, the hook answers with the configuration's
refusal. The hook starts emptying the home only after logout exits with status 0.
An error reply from this plain hook leaves the outcome of the effect unknown and keeps
shutdown from being confirmed (IFC-010, OPS-022, LIF-024).
A further retirement can dispatch it again only after reconciliation records the outcome
as `failed` (OPS-012, OPS-014, OPS-024, TPL-018). The answers carry fixed words and numbers
and any applicable configuration refusal. The client's output and the home's contents
stay out of the answers.

The hook needs the state directory. The wrapper reads `turnDeadlineSeconds`, `directory`
and `effort` after it, so the hook can still sign out an instance if the wrapper refuses
one of those inputs. If the configured directory disappears after startup, logout falls
back to the wrapper's working directory or, if that directory is also gone, to the Agent's
checked home. If the wrapper refuses an input read before the state directory, such as
`accountServices` (`AGENT_ACCOUNT_SERVICES`), the hook returns that configuration refusal.
The engine always sets `CONDUIT_URL` and `CONDUIT_IDENTITY_DIR`. Without either, the wrapper
cannot tell that it is `.v2`, lists no `retire`, and answers the call as an unknown tool.
The hook waits for a turn that is still running when it arrives.

Claude Code's pages state that logout removes and revokes the credential a keyless
Console sign-in wrote. For a claude.ai login the pages state no revocation. The hook runs
Claude Code's own logout because only that client knows where it keeps the login for
that directory. [Order 050g, L2](../../experiments/027-a5-sign-in/results-2026-11-17-logout-source.md#l2-whether-it-revokes-anything-and-the-scope)

The logout evidence comes from a [source and documentation read under item 8](../../experiments/027-a5-sign-in/results-2026-11-17-logout-source.md#the-outcome-under-item-8).

Limits:

- The hook takes any exit status other than 0 as a failure. The logout's exit status in a
  home with no login is unknown. If it exits with a nonzero status there, retirement of an
  Agent that was never signed in, for example one whose `account` is null, would remain
  unconfirmed. Any attempt dispatched after reconciliation to `failed` would encounter
  the same exit status if the home still holds no login.
- A turn that holds the session past the engine's call timeout (120 seconds by default)
  leaves the hook's outcome unknown and can leave the login in place.
- The pages do not say whether `claude auth logout` in one `CLAUDE_CONFIG_DIR` leaves
  an Anthropic profile outside it untouched. Claude Code keeps a keyless Console sign-in
  in that profile. [Order 050g, L3](../../experiments/027-a5-sign-in/results-2026-11-17-logout-source.md#l3-one-login-for-each-home)
- A login on a machine the hook cannot reach, as on a node declared lost (PLC-027), stays
  until it is revoked at the vendor. Under another runtime provider, the hook runs on
  the instance's node, where the run runs.
- The hook empties the home and keeps its directory. The engine's own runner removes it
  with the instance (PRV-029, LIF-035).

## The .v2 account and sign-out hooks

The `wrapper-v2` item implements `account-hook.v1` and `sign-out-hook.v1` as the tools
`account` and `sign-out`. Both take no arguments. The `.v1` item lists neither tool.
`claude-code.v2` states `signIn` and lists both hooks under `requires`, with the `wrapper-v2`
item as each one's default ([the sign-in hook](#the-v2-sign-in-hook)). The engine dispatches
these hooks only as parts of `sign-in.v1` or a setup for a Type that states `signIn` (CTR-013,
PND-011).

1. The account hook checks the home as every launch does, with the same refusals, and
   creates it if it is missing.
2. It runs `claude auth status` with `CLAUDE_CONFIG_DIR` set to the home and the launch's
   environment. It accepts up to 65,536 bytes of standard output and reads one extra byte
   to detect longer output. It discards standard error and waits at most 30 seconds.
3. It answers `{"account": A}` when the status exits 0 and its output is a JSON object
   with `loggedIn` true and a non-empty string `email`. It reports A exactly as the
   status gives it. It answers `{"account": null}` when the status exits 1 with
   `loggedIn` false.

The account hook leaves a running conversation's client running and retains none of the
client output beyond the account it returns. Its failures use fixed texts.

- `client status did not start`. If the wrapper accepted a named directory at startup and
  it later disappeared, the status fails to start, since the status, unlike the sign-out
  hook's logout, runs in the client directory alone. If the wrapper refused the named directory
  at startup but accepted the state directory, the client directory is the wrapper's working
  directory, and the status runs there.
- `client status failed: client exit: status N`, `client status failed: client exit: signal N`,
  or `client status failed: client exit: status unknown`.
- `client status failed: no exit within 30 seconds`. At the deadline it kills the status's own
  process group.
- `client status unreadable`. The output exceeds the limit, fails to parse as a JSON object,
  lacks a boolean `loggedIn`, or has a `loggedIn` value that disagrees with the exit status.
- `client status names no account`. The status reports a login without a non-empty
  string `email`.

The home checks give the launch's home refusals. When the configuration refused an unset
or relative state directory, the hook gives that refusal. Its answers carry fixed words
and numbers and, on success, the account alone.

1. The sign-out hook stops the client's process tree as a failed turn does. If a
   conversation's client was running, the hook ends that conversation. Every later turn
   of that conversation is refused with
   `session failed at turn K: client signed out; implicit restart refused`, until a
   [new conversation](#a-new-conversation-both-versions) starts the next.
   If a process outlives the stop or the stop is unconfirmed, the hook fails with the stop's note.
2. It checks the home as the account hook does.
3. It runs `claude auth logout` as [the retire hook](#the-v2-retire-hook) does, including
   the directory fallback and the 60-second deadline. On success it answers
   `client signed out of this Agent's own home`.

The sign-out hook keeps the home and every other entry in it, unlike the retire hook. Its
failures are the retire hook's logout failures. The hook itself removes nothing before the
client's logout runs, so a failure before the logout starts leaves the login in place; after a
failed or timed-out logout, the account hook reports whether a login remains.

Claude Code's [CLI reference](https://code.claude.com/docs/en/cli-reference#cli-commands),
section "CLI commands", read on 2026-11-18, says that
`claude auth status` emits JSON by default, includes an `authMethod` field, and exits 0 with
a login and 1 without one. The `--text` flag selects text output.

[A5's stage 0](../../experiments/027-a5-sign-in/results-2026-11-16.md#pass) recorded
Claude Code 2.1.318 naming the account in the field `email` and exiting
0 with a login and 1 in an empty home. With JSON output, the recorded field identifies a
top-level key. The hook assumes a top-level boolean spelled `loggedIn`. Its spelling and
location are unspecified in both sources. A status with a login flag of a different form
fails as `client status unreadable` and cannot complete sign-in. The engine
compares the returned account with the expected account, ignoring ASCII case (PND-011).

The status reports the client's stored login. The hook makes no independent service request
and ignores the login kind in `authMethod`. Signing out while a conversation's client is
running ends that conversation for the rest of the run.

## The .v2 sign-in hook

`claude-code.v2` states `signIn`: its account place is `/account`, and it states two logins,
each with `carried: false`: `browser`, a redirect login, and `code-page`, a page login
([the code-page login](#the-code-page-login)). Its `requires` lists `sign-in-hook.v1`,
`account-hook.v1`, `sign-out-hook.v1` and `retire-hook.v1`, each with the `wrapper-v2` item as
its default in `implementation`. A string at `/account` is the expected account, and null
disables both sign-in actions and the account check (CTR-013). The engine dispatches the hooks
only as parts of `sign-in.v1` or of a setup (CTR-013, PND-011).

The logins' literals, the authorization URL and the client ID that the two share, the browser
login's redirect host and path, and the code-page login's page, are in `contracts.yaml` alone.
The browser login's redirect states `ports: any`, since the client's listener takes a port that
the system assigns at each start.

The `wrapper-v2` item serves `sign-in-hook.v1` as the tool `sign-in`, which takes no arguments.
The request must declare `{"elicitation":{"url":{}}}` in
`_meta.io.modelcontextprotocol/clientCapabilities` (IFC-045). Without that declaration the
wrapper answers `-32021` with URL elicitation in `data.requiredCapabilities` before starting
the client. The `.v1` item lists no such tool.

1. The sign-in hook checks the home as every launch does, adding ` (PND-010)` to a
   home refusal, and stops a login that an earlier call of the hook started and that still waits.
2. It runs `claude auth login` with `CLAUDE_CONFIG_DIR` set to the home and the launch's
   environment, with `BROWSER` set to the wrapper's own program, and with
   `AGENT_SIGN_IN_CAPTURE` naming a file in a new directory, `sign-in-capture`, directly inside
   the state directory. The hook refuses a capture directory that overlaps the resolved home,
   and removes one that an interrupted earlier call left. It passes neither `--console` nor
   `--email`, and discards the client's standard input, output and error. Run as that browser,
   the wrapper keeps the first URL the client hands over in that file, with mode 0600.
3. It waits at most 30 seconds for that URL, reads at most 8,192 bytes of it, and removes the
   directory. It answers an input-required result with one URL request under the key `login`:
   mode `url`, the message `Sign the client in to this Agent's own home.` and the URL. It gives
   no request state. The engine checks the URL under IFC-049 and writes its own message.

The login keeps running after the answer and waits at its own loopback listener for the
person's browser to return there, on the Agent's machine or over a connection to it. No return
passes through Conduit (`carried: false`). The wrapper stops the login, with its process
group, after 600 seconds or once the login's own process exits, and earlier with a failed turn,
a sign-out, a retirement, the wrapper's end or a later call of the hook, since the login is part
of the client's process tree. A running conversation's client keeps running. The hook does not
show that a sign-in completed: the account hook does (PND-011).

Its failures use fixed texts, each naming PND-010, as `sign-in-hook.v1` asks of a hook that
cannot start a login:

- `sign-in hook: a .v1 Agent has none (PND-010)`.
- The shared configuration refusal, when the state directory is unset, or any of the
  launch's shared home refusals, with ` (PND-010)` appended.
- `client login did not start: could not resolve state directory (PND-010)`.
- `client login did not start: capture directory overlaps client home (PND-010)`.
- `client login did not start: could not create capture directory (PND-010)`.
- `client login did not start: client process tree is closed (PND-010)`.
- `client login did not start: could not spawn client (PND-010)`.
- `client login did not start: client process ID unavailable (PND-010)`.
- `client login failed: client exit: status N (PND-010)`,
  `client login failed: client exit: signal N (PND-010)` or
  `client login failed: client exit: status unknown (PND-010)`, where the login ends before or as
  it hands its URL over.
- `client login failed: no URL within 30 seconds (PND-010)`.
- `client login gave no usable URL (PND-010)`, for a URL longer than the limit, with a space or a
  character that is not ASCII, or not beginning `https://`, or one handed over as anything but
  a regular file, such as a link or a FIFO.

On failure the hook stops any login group it started and removes any capture directory it
created. A call that carries a request state or input responses is answered
`sign-in hook: this login carries no return (PND-013)`. The hook relays
only the URL handed to the browser: the client's printed output, which holds the code-page
login's URL, is discarded, and no answer, diagnostic or log of the wrapper holds any of it.

The logins and the hand-over were read from Claude Code 2.1.318's program, `bin/claude.exe` of
its npm package at that version, the macOS arm64 build, by identifier: the OAuth
configuration's `CLAUDE_AI_AUTHORIZE_URL`, `CLIENT_ID` and `MANUAL_REDIRECT_URL`; the
authorization URL's builder, whose loopback `redirect_uri` gives the browser login's host and
path and whose other `redirect_uri` is `MANUAL_REDIRECT_URL`, the code-page login's page; the
callback listener, started with no port; the OAuth flow, which builds both URLs from one state
and one challenge, gives the code-page URL to its caller and opens the browser login's; the
opener, which runs `BROWSER` with the URL as its one argument; and `auth login`, which takes the
claude.ai login unless `--console` is given, prints the code-page URL and reads a pasted code,
with its state, as a line of its standard input.
[A5's stage 0](../../experiments/027-a5-sign-in/results-2026-11-16.md#pass) recorded on macOS
that Claude Code 2.1.318's login, started with no terminal, handed its browser a loopback
redirect URL that passed IFC-049's checks while its listener accepted connections, and printed a
second URL, a page login's, for which the probe's IFC-049 checks found no failure.

Limits: the `wrapper-v2` item is `plain` in this release, so the hook gets no Conduit keys
(IFC-007), and the item is keyed only in a later release (ruling 86, points 1 and 2). A plain
hook's URL request inside `sign-in.v1` is still relayed to the person, under IFC-007, IFC-016,
IFC-010 and PND-010 (ruling 86, point 3). The engine refuses a `sign-in.v1` request that asks
for a login by name (`signInLogin`) for this plain hook before any part runs, so no browser
opens (PND-010, ruling 86, point 3); without a name, the hook starts the browser login. Only
the hand-over on macOS was observed, and a real client's closeout is still required.

### The code-page login

`code-page` is the second route of the same `claude auth login`. From the state and challenge of
the browser login's URL, the client builds a URL whose `redirect_uri` is the service's own HTTPS
page, prints it, and waits for the code that the page shows, pasted with its state as a line of
the login's standard input. Its `return` is `page` and it states `carried: false`: no Conduit
process carries, holds or records the code, and the person pastes it on the Agent's own machine,
at it or over a connection to it (CTR-013, PND-013).

The Type states it now, since a login that this release does not state never joins
`claude-code.v2` (ruling 82, point 4). It cannot be chosen in this release. The `wrapper-v2` item
is `plain`, so the engine refuses a `sign-in.v1` request that names this login, or any other,
before any part runs (PND-010, ruling 86, point 3), and the hook, asked for no login, starts the
browser login. The hook discards the client's standard input and output, so it relays no
code-page URL, and the login it starts takes no pasted code. Choosing the code-page login waits
for a later release, whose keyed item gets the login's name (ruling 86, points 1 and 2).

## A new conversation (both versions)

From 0.2.1, `conversation.v1` includes the operation `new-conversation.v1`, which the wrapper
exposes as `new-conversation`. It takes no input and starts no model work. A Conduit can grant
either operation on its own or grant both.

The call waits for a running turn or hook. It ends the conversation and stops the client as at
a failed turn, including the whole process tree in `.v2`. It forgets the conversation's thread,
failure and turn count. Client launch waits for the next `turn`, which resumes no earlier
conversation.
It launches a new Claude Code process. The wrapper runs Claude Code with
`--no-session-persistence`, so the client keeps no ended conversation to resume.
Files in the Agent's directory and home stay, including anything the client stores there. The
Agent's run and the engine continue. After a failed turn, the wrapper keeps refusing turns in
the ended conversation. An explicit `new-conversation` starts its successor.

The reply has the shape
`{"conversation":N,"ended":{"turns":T,"failed":F,"survivors":[...],"stopConfirmed":B}}`.
N is the new conversation's number within this wrapper run. A run starts at 1. T counts the
turns the ended conversation started. F tells whether a turn failed or was cancelled. The
[failed turn's fields](#the-v2-failed-turns-fields) describe how the wrapper checks for survivors
and confirms the stop. Here `stopConfirmed` is true when the stop was confirmed and no processes
survived. In `.v1`, `survivors` is empty and `stopConfirmed` is true. If `survivors` is non-empty or
`stopConfirmed` is false, end the Agent's run before sending another turn. Processes from the
ended conversation may still be running.

- An input returns invalid parameters with `expected no arguments`.
- Invalid configuration returns the same `invalid configuration: ...` text as a turn.
- A retired Agent returns `session retired; no new conversation starts`.
- While a `.v2` sign-in runs, the login continues and the call returns
  `new conversation refused: a sign-in is in progress in this Agent's own home; nothing was stopped; call again once the sign-in has ended`.

Through a Conduit, the engine rejects an input as invalid with no outcome before calling the
wrapper. The wrapper's configuration, retirement and sign-in refusals give an unknown outcome
with the refusal text, as a turn failure does, because the contract states an effect and the item
is plain (IFC-010).
Use a fresh idempotency key to retry.

The 0.2.1 contracts extend 0.2.0 only with NAM-024's additions. They add the operation to
`conversation.v1`, its Operation contract and its default implementation in both Types.
The same configuration and state directory
select the same Agent home as in 0.2.0. The engine determines whether an installed Agent keeps
its instance and sign-in when its pin moves to 0.2.1.

## The account home Resource

`account-home.v1` is a Pool-owned Resource for one Claude Code account on one machine. Set
`config.account` to the expected account. The Resource works with any Claude Code subscription
plan. Its hooks create a private `account-home` in its state directory and use that directory
as `CLAUDE_CONFIG_DIR` for `claude auth login`, `claude auth status` and `claude auth logout`.
Its retire hook logs out and empties the directory. Only the Resource's own hooks perform
those login operations. Set up the Resource and complete `sign-in.v1` through its owning Pool
before setting up its Agents.

## The .v3 Agent

Set `config.login` on a `claude-code.v3` Agent to its `account-home.v1` Resource. Use the
engine's runner for both and place them on the same node. Leave `runtime` unset. A custom
runtime or different placement prevents setup under PRV-029. The runner passes the Resource's
state directory as `AGENT_LOGIN_DIRECTORY`. Each launch sets `CLAUDE_CONFIG_DIR` to the
Resource's existing `account-home`.

Each Agent keeps its own conversation, settings and MCP servers. The wrapper passes
`--no-session-persistence`, so the client keeps no transcript. `settings.json` and `mcp.json`
for the run are in its private working directory. Auto memory would use the shared home.
Both `.v2` and `.v3` disable it through `autoMemoryEnabled: false` and
`CLAUDE_CODE_DISABLE_AUTO_MEMORY=1` on every launch.

The home shares `.claude.json` for account details and project trust, and `stats-cache.json`
for `/usage` totals. DEC-047 measures usage per turn. Per-session files under `file-history/`,
`session-env/` and `sessions/` are readable by every Agent of the home (PRV-029, DL16). An
Agent's own unconfined tools can sign the account out (ruling 70).

If the account home is missing or the client reports an authentication failure, the turn
reports a lapse through IFC-016. The engine raises one sign-in action and one one-at-a-time
notice on the Resource. The report holds no Agent. Root can sign in through the owning Pool
and acknowledge the notice to lift the one-at-a-time rule. The Resource's own sign-in hold
holds model work for all its Agents. The `.v3` retire hook stops its client and leaves the
shared directory and login to the Resource.

## The .v4 Agent

`claude-code.v4` adds `instructions` and `outputSchema` to the shared-login Agent. Set either
place to `null` to omit it. Registration rejects an empty string. The limits leave room for
four UTF-8 bytes per character, the variable name, the equals sign, and the terminator within
Linux's 131,072-byte limit per environment entry.

- `instructions` takes 1 to 32,763 characters. The wrapper writes it byte for byte to
  `instructions.md` in the run's private directory and passes the file to
  `--append-system-prompt-file`. A link at that destination is refused.
- `outputSchema` takes 1 to 32,762 characters. Write JSON text because the run variable
  carries a string (MAN-017). The wrapper checks that it parses as an object with
  `"type": "object"` and passes the original text as one `--json-schema` argument. If that
  check fails, every turn reports `invalid configuration: outputSchema (AGENT_OUTPUT_SCHEMA):
  expected the JSON text of a schema whose type is object` and starts no client.

When Claude Code returns an object in `structured_output`, the turn returns it in
`structured` and leaves `text` unchanged. The subtype
`error_max_structured_output_retries` fails the turn with class `structured-output`.

## The .v5 Agent

`claude-code.v5` uses the `.v4` configuration and accepts the same `login` Resource.
It requires `setup-hook.v1` before `retire-hook.v1` and defaults each operation and hook
to `wrapper-v5`. The item uses the same command and bindings as `wrapper-v4` and maps
the setup hook to `setup`. Setup checks the client before any turn.

## Usage in each turn's result

The wrapper reports per-model usage from the client's `modelUsage` in
`_meta["io.github.shrav-k.conduit/usage"]` (IFC-051). The engine keeps it in the call's end
record. Both successful and failed client results can report usage. The client version
comes from the latest `system/init` event in that process.

The first report uses the client's totals. Later reports subtract the same model's last
cumulative counts in that process. A model absent from the previous report starts at its
reported totals. Models whose deltas are all zero are omitted. `input` includes input,
cache reads and cache creation. `cachedInput` counts cache reads, and `reasoning` is the
thinking already included in `output`. When a cache counter appears or disappears for
the same model across two reports, that model is omitted for the turn because its total
input delta cannot be measured. A reasoning counter is omitted when its delta cannot be
measured. After a missing or malformed `modelUsage` report, the next valid report
establishes a new baseline without emitting usage. A new conversation starts a new
process and baseline.

There is no usage member without a valid client version or a usable `modelUsage` report,
if a cumulative count fell, or if the report would contain zero models or more than 16.

## A usage limit's window and reset

On a client failure, a `rejected` rate-limit event for the turn gives class `usage-limit`.
An `allowed_warning` leaves the turn unchanged. A turn with a successful result still
succeeds after a rejected event. Text in the client's `result` or `errors` array identifies
the window as `session`,
`weekly`, `opus` or `sonnet`. Without any of those words the window is `null`.
`windowMinutes` is always `null`.

The last rejected event's `resetsAt` is interpreted as seconds and as milliseconds since
the Unix epoch. If exactly one reading falls after the start of the turn and within
eight days, the report includes that time in RFC 3339 UTC. Otherwise `resetsAt` is
`null`. The wrapper makes no additional request and uses no reset time from the text.
DEC-046 pauses only that account's lanes until the reported reset.

## Model, effort and what stays off

| Place or setting | Client argument or setting | Default or effect |
| --- | --- | --- |
| `model` | `--model` | `''` uses the client's default model |
| `effort` on `.v2` and later | `--effort` | `''` uses the client's default effort |
| Auto memory on `.v2` and later | `autoMemoryEnabled: false` and `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1` | Off on every launch |
| Hooks and plugins | `disableAllHooks: true` and `enabledPlugins: {}` | Off |
| Sub-agent tools | Omitted from `--tools` | Off |
| Slash commands and skills | `--disable-slash-commands` | Off |
| Setting sources | `--setting-sources ""` | User, project and local settings omitted. Whether project `CLAUDE.md` is read awaits the first real turns. |

## The vendor client

The lowest client version this release was tested with is Claude Code 2.1.318.
The package's digest covers the wrapper and this bound, not the client.
The wrapper finds `claude` on the PATH of the machine where it runs (PRV-019).
Use the vendor's instructions to install or update the client. The native install updates
itself. The wrapper leaves that behavior in place. [Claude Code setup](https://code.claude.com/docs/en/setup)

Claude Code `.v5` checks the client. The other Types and login Resources do not run the
check (TPL-012).
The setup hook runs one `--version` with a five-second deadline and a 256-byte output limit.
Here `P` is the absolute path of the client found and `V` is its reported SemVer version.

```text
client: claude V at P; lowest tested 2.1.318
client: claude V at P is older than 2.1.318, the lowest version this release was tested with; update it by the vendor's own route
client: claude at P gave no version this release can read from --version; the lowest version it was tested with is 2.1.318
client: claude not found on PATH; install 2.1.318 or later by the vendor's own route
```

The last three results fail the hook. The engine names the hook part in its failed setup reply.
A later engine change will include the hook text in that reply. A failed hook runs again at
the next setup. A succeeded hook is kept, so replace an Agent to check a later client (OPS-021,
OPS-028).

## The standard Conduit templates

From 0.2.2, `turn` grants `turn.v1` alone, and `conversation` grants both `turn.v1` and
[`new-conversation.v1`](#a-new-conversation-both-versions). Write
`operator: {from: example.org/conduit/agent-claude-code#turn}` to grant root turns. Write
`operator: {from: example.org/conduit/agent-claude-code#conversation}` to let root also end a
conversation. A later release keeps each template's operation list. A wider grant takes a new
template (PKG-031).

## Moving to 0.2.3

Install the 0.2.3 archive, move the pin, and replace each Agent that should use the new release.
An instance keeps the release it resolved. A replaced `.v2` Agent signs in again. To move to
`.v3`, retire each `.v2` Agent first to log out its own login. Confirmed shutdown removes the
old Agent automatically. If it remains retired, wait for shutdown confirmation or unregister
it before reusing the name. A tombstone can also block name reuse (LIF-041). Set up and sign in
each account's Resource once per machine, then register and set up the `.v3` Agents that refer
to it.

## Moving to 0.2.4

Install the 0.2.4 archive, update the package pin, and replace each Agent that should use
the new release. A `.v4` Agent that replaces a `.v3` Agent can use the same `login`
Resource without another sign-in.

## Moving to 0.2.5

Install the 0.2.5 archive, move the project pin to its version and digest, and replace each
Agent that should check its client. A Claude Code `.v4` Agent replaced by a `.v5` Agent
with the same `login` does not sign in again.
