# agent-codex

Maintainer: the Conduit project's owner. Lowest tested client: Codex 0.167.1. Work orders 023
and 024 tested this package with its scripted client only.
The admin's [real-client closeout](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/first-build/increment-3/closeout-results-2026-11-10.md)
and [increment 4 log](../../docs/implementation/first-build/log-increment-4.md) record subsequent trials on 2026-11-10.

This standalone Rust crate has no dependency on another repository directory. Its lockfile
and pinned toolchain travel with it. Its manifest exports five Agent Types, a login Resource
Type, a conversation capability with `turn.v1` and `new-conversation.v1`, and a history-reading
capability with `history.v1`. The plain Agent wrapper exposes `turn` and `new-conversation`.
The `.v5` item also exposes `history`. Each
`.v2` item also exposes the hook tools `retire`, `account`, `sign-out` and `sign-in`.
The manifest maps each versioned operation or hook to its tool. In `.v1`, both reach inputs
are required booleans.
Authors may omit `model`: the engine fills its empty-string default, leaving the native
client's default in force. MAN-017 requires the bound scalar to be required in the resolved
configuration, so the Type's required list includes model. This is the logged nearest sound
reading of work order 023's optional model input.
Agent setup initializes the wrapper and starts no model work. Inputs and outputs are
excluded from Conduit records by the Types.

Run these commands in this directory:

```sh
cargo test --release
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo run --release --bin package
```

Built offline on the same host with the same toolchain, the archive has the same digest on
every run.

The last command builds the host wrapper and writes `dist/agent-codex.tar.gz` and its `.sha256`
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

In `.v1`, Codex resumes the exact thread in a fresh process per turn. Prompts are written verbatim
to stdin with the `-` prompt argument, including literal `-` and option-like text. `--ignore-user-config`,
`--strict-config` and `--ignore-rules` exclude user configuration/rules;
`-c project_doc_max_bytes=0` also prevents ancestor/project AGENTS.md discovery, which
`--ignore-rules` alone did not stop (finding 3-05). The sandbox is read-only and web search
disabled. Every launch and exact-thread resume disables `shell_tool`, `unified_exec`,
`browser_use`, `browser_use_external`, `in_app_browser`, `computer_use`,
`multi_agent`, `multi_agent_v2`, `multi_agent_v2_dynamic_tools`, `hooks`, `plugins`, `view_image`, `goals` and `image_generation` with
`--disable`. It leaves `code_mode_host` on: Codex calls MCP tools, the engine's among them,
through it, and its code execution imports nothing that reaches the machine (finding 3-06).
Apps are explicitly disabled unless accountServices is
true, when they are explicitly enabled. Codex has no separate peer-messaging launch switch
in the qualified baseline; peerMessaging is accepted/bound but changes no Codex argument.
The HTTP server is required and its default_tools_approval_mode is approve. Its header helper
receives the identity directory on its command line because Codex removes the run environment
from helpers. No env_vars or inline token is configured for that server.

The `.v1` launch disables the client's shell, browser, hooks and plugins, alongside account services
and messaging. The collaboration tools remain available, as finding 4-06 below states.
`accountServices: true` enables `apps` alone; none of the disabled features is turned back on.
The admin's direct trial still listed `apply_patch` (in the read-only sandbox) and a request
for user input. These remain routes the closeout must report and exercise where possible,
along with any other native tool still exposed; their presence is not engine authorization.

**Mediated access (ACC-027), for `.v1` and `.v2` with no directory:** the former read-only shell bypass is disabled by these launch
arguments. Closeout must ask Codex to read an outside sentinel without engine tools and
observe the result, not infer confinement from absent engine requests. Shared OS identity,
access to other run credentials and, for `.v1`, the shared login still prevent strong
isolation. Any opted-in app or other remaining native tool must have its reach stated; the
collaboration tools below stay within the Agent's own access.

Work order 027 keeps version 0.1.1 and disables both multi_agent_v2 and
multi_agent_v2_dynamic_tools as well as multi_agent on initial turns and resumes, regardless
of either reach input. The bundled 0.167.0-alpha.3.2 binary's feature list names both;
its strings include the collaboration tools. That binary rejected --ignore-user-config,
so the metadata-only feature listing used an empty temporary CODEX_HOME instead. With the
three explicit disable flags, the feature listing reports all three false. No client session
or model work ran for that metadata check.

The real-client follow-up in [order 027's Record](../../docs/implementation/WORK_ORDERS.md#order-027)
corrected that metadata-only observation (the open status below is historical):

Decision 6 did not hold with the real client: with `multi_agent`, `multi_agent_v2` and
`multi_agent_v2_dynamic_tools` disabled, and in further trials `collaboration_modes`,
`in_app_local_automation` and `agents.max_depth=0`, Codex 0.167.0 still listed its six
`collaboration.*` tools. Finding 4-06 stays open. The trials also showed `sleep_tool` controls
`clock.sleep`, and that a trial without the wrapper's `web_search="disabled"` lists `web__run`.

**Finding 4-06, settled by the admin on 2026-11-10:** Codex 0.167.0 keeps all six tools,
`collaboration.spawn_agent`, `collaboration.send_message`, `collaboration.list_agents`,
`collaboration.wait_agent`, `collaboration.interrupt_agent` and `collaboration.followup_task`,
under every switch tried: the three `multi_agent` switches above, `collaboration_modes`,
`in_app_local_automation` and `agents.max_depth=0`. In the admin's trial with the wrapper's
flags, a spawned sub-agent had no shell, could not read outside its sandbox and had an
`apply_patch` write refused by the read-only sandbox. Its requests of the engine carry the
parent Agent's credential, so it reaches only what that Agent may. This is a route to more
parallel model work within the Agent's own access, not around it. What offers the tools is
still not found; the [log](../../docs/implementation/first-build/log-increment-4.md) records the
trial and settlement.

## The .v2 Agent

The package also exports codex.v2, implemented by the separate `wrapper-v2` item.
A template may select `wrapper-v2-path` instead, as [its section](#the-wrapper-v2-path-item) says.
All properties are required in the resolved instance configuration after SCH-009 fills their defaults.
A null `directory` gives no environment variable (PRV-007).

| Input | Default / bounds | Wrapper variable |
| --- | --- | --- |
| `directory` | `null`; otherwise an absolute path to an existing directory | `AGENT_DIRECTORY` |
| `turnDeadlineSeconds` | `180`; integer from 1 through 604800 | `AGENT_TURN_DEADLINE_SECONDS` |
| `effort` | `''`; otherwise lowercase ASCII letters | `AGENT_EFFORT` |
| `accountServices` | `false` | `AGENT_ACCOUNT_SERVICES` |
| `model` | `''` | `AGENT_MODEL` |
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
The Type's description names no client version (ruling 82, point 5); this README records the version read.

When the configuration names a directory, nothing confines the client in v0. Its shell, code
execution, file edits and network run as the engine's operating-system user, with the
client's sandbox and approval prompts off. It can reach every file, process, network address, credential and
signed-in account that user can reach: the engine's store and root's client token;
`CONDUIT_IDENTITY_DIR`, with this run's token; the default home of the engine's user, with
its login, and every other Agent's own home in the engine's store. It can push through the keychain,
whatever HOME is, and reach other git worktrees that share one `.git`. This reach does
not pass through Conduit: no Conduit grants, limits or records it (ruling 70).
Codex 0.167.0 still offers its collaboration tools (finding 4-06), even with its
multi-agent features disabled; their subprocesses inherit this permissive reach.
The shell can reach services, the user's browser and desktop even with
`accountServices: false`; that input shapes only the client's tool list. The mediated
access paragraph above holds only for `.v1` and for `.v2` with no directory.

The wrapper checks that `directory` is absolute and exists, then canonicalizes it.
The client starts there, or in the run's private directory when it is null.
`Config::cwd` remains the run's directory. Work orders explicitly tell workers to read
AGENTS.md; automatic project instruction discovery stays off.

Codex launches `exec --sandbox S --color never`, then optional `--model M`, then
`--cd DIR` on every turn. S is `danger-full-access` with own tools, `read-only` otherwise.
A resumed turn puts `resume` after these flags, before
`--ignore-user-config --strict-config --ignore-rules --skip-git-repo-check --json`.
Apps get `--enable apps` or `--disable apps` according to accountServices.
With own tools, `--disable` still names, in order, `browser_use`, `browser_use_external`,
`in_app_browser`, `computer_use`, `multi_agent`, `multi_agent_v2`,
`multi_agent_v2_dynamic_tools`, `hooks`, `plugins`, `goals`, `image_generation`.
With own tools off, `.v1`'s fourteen disabled features above remain.

The seven `-c` settings remain the engine URL, header helper, startup readiness,
required server, default tool approval, web search and project document size. Search is
`web_search="live"` only when accountServices is true and a directory is named; otherwise
it is `web_search="disabled"`. `project_doc_max_bytes=0` stays.
Then `-c cli_auth_credentials_store="file"`, then `-c mcp_servers.engine.tool_timeout_sec=N`
equals turnDeadlineSeconds, followed by optional `-c model_reasoning_effort=E` with E
JSON-quoted, then the resumed thread if any, then `-`. Codex writes no settings.json or
mcp.json. Its client environment is `.v1`'s, plus `CODEX_HOME`.


`conduit serve --call-timeout-seconds` must be longer than `turnDeadlineSeconds`.
With their defaults, 120 and 180, the engine reports a turn longer than 120 seconds
before the wrapper's deadline. Longer turns may also require the engine fix in order 074.

## The .v2 Agent's own home

The `.v2` wrapper requires the state directory that the engine's runner gives each
instance (PRV-029). It reads `CONDUIT_STATE_DIR` from its own environment alone. No input
sets the variable or the home. The wrapper creates `client-home` in that directory
with mode 0700 on Unix when it is missing. At every launch it checks the home and sets
`CODEX_HOME` to its canonical path. `HOME` still comes from the runner.

The wrapper reports these refusals.

- `client home (CODEX_HOME): CONDUIT_STATE_DIR is unset`
- `client home (CODEX_HOME): CONDUIT_STATE_DIR is not an absolute path`
- `client home (CODEX_HOME): CONDUIT_STATE_DIR is not an existing directory`
- `client home (CODEX_HOME): cannot create the home in CONDUIT_STATE_DIR`
- `client home (CODEX_HOME): the home resolves outside CONDUIT_STATE_DIR`
- `client home (CODEX_HOME): the home is not a directory`
- `client home (CODEX_HOME): the home holds a .env, which Codex would read into its environment; the wrapper refuses it, since the Agent holds no API key (ruling 83)`
- `client home (CODEX_HOME): cannot check the home for a .env`
- `client home (CODEX_HOME): the canonical home path is not valid Unicode`

If a runner supplies no state directory, every `.v2` turn fails with the unset diagnostic.
A `.v1` launch works as in 0.1.1 and uses the engine user's default client home.
The engine's own runner removes the home with the instance (PRV-029, LIF-035).
Before that, the [retire hook](#the-v2-retire-hook) signs Codex out and empties the home.
Under another runtime provider, that provider's contract states what happens to the home.
The login in the home belongs to Codex, which keeps it where it stores logins for
that directory.

Every `.v2` launch passes `-c cli_auth_credentials_store="file"` because
`--ignore-user-config` skips the home's `config.toml`. The setting makes Codex keep each
Agent's login in a file in its own home, so two Agents' logins never share a store, unless
a managed requirement supplies the setting, which then takes precedence
([order 197](../../experiments/027-a5-sign-in/results-2026-11-17-account-read-source.md#r1-whether-the-app-server-offers-accountread-and-how-it-is-reached)).

Codex 0.167.1 treats a non-Unicode `CODEX_HOME` as unset and falls back to the engine
user's default Codex home
([order 197](../../experiments/027-a5-sign-in/results-2026-11-17-account-read-source.md#r1-whether-the-app-server-offers-accountread-and-how-it-is-reached)).
The wrapper refuses such paths before launching Codex.
Codex reads the home's `.env` into its own environment after the wrapper launches it.
The file can set any variable whose name does not begin `CODEX_` in any ASCII case,
including an API key, past the wrapper's environment list. Only the Agent's own unconfined
tools or a process of the engine's user can write that file (ruling 70).
No template gives a `.v2` Agent an API key: the Type states no `apiKey` (ruling 83), and nothing falls back from a sign-in to a key (ruling 82, point 1).
Every `.v2` launch and every hook once one exists refuses a home that
holds a `.env`.
If a `.env` is written between the check and Codex's read, that launch alone reads it and
the next launch refuses it if it is still present. Codex creates `tmp/` in the home for
its helper aliases. A release build may skip the aliases for a home under the system's
temporary directory and emit a warning that the wrapper discards. Order 197 left open
whether the comparison would match a given directory. [Order 197's source reading](../../experiments/027-a5-sign-in/results-2026-11-17-account-read-source.md#r4-what-answering-reads-writes-refreshes-or-calls)

## The .v2 client's process tree

Each `.v2` launch starts Codex as the leader of a new process group. On a failed turn,
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

- The wrapper samples the tree during a stop, and while a sign-in hook's login runs, every 250
  milliseconds. A process that left the client's group and lost its parent links to the tree
  between two samples, or before a stop's first read, is neither stopped nor named. A double fork that also starts a new session can do this. If Codex exits before the stop,
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

## The wrapper-v2-path item

`wrapper-v2-path` is `wrapper-v2` with `command: [agent-codex]`. It binds the same inputs
to the same variables, so everything this README says of `.v2` holds for it.
The defaults of `codex.v2` for `turn.v1`, `new-conversation.v1` and its four hooks are still `wrapper-v2`.

`bin/agent-codex` is a relative path, a file of the archive on the engine's machine
(MAN-010). The engine asks no runtime provider but its own runner to run such an item
(PRV-007). The archive's binary is a host build.

`agent-codex` holds no `/`, so it is a name, found as the one who starts it finds programs
(MAN-010). Under a runtime provider other than the engine's own runner, that is on the
machine of the instance's node (PRV-019), as that provider's contract states. The wrapper
then finds `codex` on its own PATH, as it always does. On the engine's own runner, the item
runs whatever `agent-codex` is found on the engine's machine, not the archive's: use
`wrapper-v2` there.

A `command` pins nothing (MAN-010). The archive's digest does not cover the program found
on that machine. Whoever prepares the machine builds `agent-codex` there from the same
source as the installed archive, and puts it and `codex` where that provider's runs find
them. MAN-010 publishes a server meant to run on another machine as a pinned `package` item.
This item is a shortcut that stands in for one, and is not one.

A template selects it by writing
`implementation: {example.org/conduit/agent-codex#turn.v1: example.org/conduit/agent-codex#wrapper-v2-path, example.org/conduit/agent-codex#new-conversation.v1: example.org/conduit/agent-codex#wrapper-v2-path, github.com/shrav-k/conduit/core#retire-hook.v1: example.org/conduit/agent-codex#wrapper-v2-path, github.com/shrav-k/conduit/core#account-hook.v1: example.org/conduit/agent-codex#wrapper-v2-path, github.com/shrav-k/conduit/core#sign-out-hook.v1: example.org/conduit/agent-codex#wrapper-v2-path, github.com/shrav-k/conduit/core#sign-in-hook.v1: example.org/conduit/agent-codex#wrapper-v2-path}`
beside its `type` and `config`. A template that selects it for some of them alone takes `wrapper-v2`
for the others by default, two items that are run, and is rejected (TPL-019).
An instance of it names its runtime provider in `runtime`
(PRV-006). An instance that names none runs on the engine's own runner, on the engine's
machine (PRV-019).

`directory` is then a path on that machine (PLC-020), and the wrapper checks it there.

The paragraph above on what nothing confines describes the engine's own runner. Under
another runtime provider, the client and its own tools run on that machine, as the
operating-system user that the provider runs them as, with that user's files. A `.v1` client uses whatever
client login that user has there; a `.v2` client's home is `client-home` in the state
directory that provider gives, and a provider that gives none fails every `.v2` turn,
naming it. That provider's contract states what, if anything, confines them. This package
copies no login.

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

CLASS uses one case-insensitive, ordered substring table; the first matching class wins:

| Class | Substrings |
| --- | --- |
| `usage-limit` | `usage limit`, `usage_limit`, `quota` |
| `rate-limit` | `rate limit`, `rate_limit`, `too many requests`, `429` |
| `authentication` | `authentication`, `unauthorized`, `401`, `login`, `log in`, `api key`, `credential` |
| `context-window` | `context window`, `context_window`, `context length`, `context_length`, `prompt is too long` |
| `network` | `network`, `connection`, `disconnected`, `error sending request`, `timed out`, `timeout`, `dns`, `getaddrinfo`, `econnreset`, `econnrefused`, `enotfound`, `unreachable`, `fetch failed` |
| `other` | No match or no text |

These substrings are an initial list tested with synthetic failures, not real failures.
Failure results, errors and stderr never include raw client text. Invalid configuration
still initializes MCP and lists `turn`; every turn reports `invalid configuration: ...`,
naming the input/variable, reason and directory path where relevant, without starting a
client or poisoning a session.

## The .v2 failed turn's fields

When a `.v2` turn fails, the result's `content` holds the failure text followed by a
second text block containing one JSON object, `{"failedTurn":{...}}`. The `failedTurn`
object has these fields.

| Field | Value |
| --- | --- |
| `turn` | The turn number, counted from 1 across resumes |
| `cause` | The fixed kind selected from the cause text below |
| `class` | The class from `client failure: CLASS`, or `null` for every other cause |
| `limit` | The window and reset for `usage-limit`, or `null` for every other class |
| `clientStarted` | Whether the session held a client when the failure was recorded, before stopping it |
| `survivors` | The PIDs that outlived the stop in ascending order, or an empty list |
| `stopConfirmed` | `true` if the stop was confirmed, otherwise `false` |

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

`codex.v2` lists `retire-hook.v1` under `requires`. Its default is `wrapper-v2`, the
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
3. It runs `codex logout -c cli_auth_credentials_store="file"` with `CODEX_HOME` set to
   the Agent's home and the launch's environment. It discards the client's output and
   waits up to 60 seconds for exit.
4. On exit status 0, it checks the home again as in step 2, and fails with
   `client home (CODEX_HOME): cannot empty the home` if the home now resolves to another
   directory, removing nothing. It then removes every entry in the home. For a link it removes the
   link itself. It keeps the home directory and answers `client signed out and its home emptied`.

The hook reports failures with these fixed texts.

- `client logout did not start`
- `client logout failed: client exit: status N`
- `client logout failed: client exit: signal N`
- `client logout failed: client exit: status unknown`
- `client logout failed: no exit within 60 seconds`. At the deadline the hook stops the logout's
  tree and appends the stop's note if there is one.
- `client home (CODEX_HOME): cannot empty the home`

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

At the tag named in the read, Codex's source attempts one best-effort revocation request
naming the home's own stored token, when the home holds one, and then removes the home's login
even if the request fails, so successful local sign-out gives no confirmation of vendor
revocation. This result comes from reading the source. [Order 050g, C3](../../experiments/027-a5-sign-in/results-2026-11-17-logout-source.md#c3-whether-it-revokes-at-the-vendor-and-the-scope)

The logout evidence comes from a [source and documentation read under item 8](../../experiments/027-a5-sign-in/results-2026-11-17-logout-source.md#the-outcome-under-item-8).

Limits:

- The hook takes any exit status other than 0 as a failure. By Codex's source at the tag named
  in the read, the logout exits 0 in a home with no login, where the configuration loads and
  neither a machine-managed configuration nor the home's config.toml sets up a cleanup that
  fails; no run of the client has observed it. If it exits with a nonzero status there,
  retirement of an Agent that was never signed in, for example one whose `account` is null, would
  remain unconfirmed. Any attempt dispatched after reconciliation to `failed` would encounter
  the same exit status if the home still holds no login.
- A turn that holds the session past the engine's call timeout (120 seconds by default)
  leaves the hook's outcome unknown and can leave the login in place.
- A login on a machine the hook cannot reach, as on a node declared lost (PLC-027), stays
  until it is revoked at the vendor. Under another runtime provider, the hook runs on
  the instance's node, where the run runs.
- The hook empties the home and keeps its directory. The engine's own runner removes it
  with the instance (PRV-029, LIF-035).

## The .v2 account and sign-out hooks

`codex.v2` lists `account-hook.v1` and `sign-out-hook.v1` under `requires`, beside the retire hook.
Both default to `wrapper-v2`, which serves them as the plain tools `account` and `sign-out`, each
taking no arguments. The `.v1` item lists neither. The engine calls these hooks only as parts of
`sign-in.v1` or of a setup for a Type that states `signIn` (CTR-013, PND-011). `codex.v2` states
`signIn` only where A5's stage 1, step 1 passed: order 050n joins this package into 0.2.0 only
on that pass, and on a fail leaves it out of 0.2.0.

1. The account hook refuses a `.v1` Agent. If the state directory is unset and configuration was
   refused, it returns that refusal. It also refuses an Agent once retirement has begun. It
   checks the home as every launch does, with the same refusals, and creates it if it is missing.
2. It starts `codex app-server --stdio -c cli_auth_credentials_store="file"` with `CODEX_HOME`
   set to the Agent's home and the launch's environment, with `CODEX_REFRESH_TOKEN_URL_OVERRIDE`
   set to `https://refresh.invalid/oauth/token` (below). It discards stderr.
3. It sends `initialize` with `clientInfo` naming `agent-codex` and this package's version and
   `experimentalApi` set to false. It sends the `initialized` notification, then `account/read`
   with `refreshToken` set to false. Each line carries one JSON message without a `jsonrpc`
   member. It skips incoming notifications and requests and waits at most 30 seconds for the two
   answers together.
4. After the exchange, it stops the app-server's process group and its descendants, following
   parent links as every `.v2` stop does. It sends TERM, then KILL if needed, with the same grace
   and wait as a failed turn. Any other recorded group, such as a login still running, is left alone.
   It closes the app-server's input only after that stop. If the app-server's output ends first, it
   reads the tree once before waiting for the exit, so a descendant reparented by the exit is still
   found. A failed reading makes the stop unconfirmed.

The hook reports the non-empty `email` of a `chatgpt` account as `account`, with the text
`account hook (account/read): Codex reported a ChatGPT account`. It reports a null account with
one of these texts.

- `account hook (account/read): no account: Codex answered with none`
- `account hook (account/read): no account: the login is not a ChatGPT login`
- `account hook (account/read): no account: the ChatGPT login names no email` for a null or empty email.

The failures use these texts. `STEP` is `initialize` or `account/read`.

- `account hook (start): client did not start: the client directory is gone`
- `account hook (start): client did not start`
- `account hook (STEP): no answer within N seconds`
- `account hook (STEP): client exit: status N`
- `account hook (STEP): client exit: signal N`
- `account hook (STEP): client exit: status unknown`
- `account hook (STEP): EOF before answer`
- `account hook (STEP): error answer, code N`
- `account hook (STEP): error answer`
- `account hook (STEP): invalid answer`
- `account hook (stop): NOTE` if a stop leaves survivors or is unconfirmed after an account
  report. If the exchange already failed, the hook appends `; NOTE` to the failure text.

Silence or an error from Codex leaves the account unknown, so the hook fails. A failed account
hook does not complete sign-in (PND-011). Before the sign-in hook, a failed check still goes on
to it, and only a later check that names the account completes sign-in (PND-010, as the owner's
ruling 86, point 3, amends it). Diagnostic texts carry only the step and fixed words and numbers.
Codex's output, error messages, claims, tokens and file contents stay out of them. Codex alone
reads the home's login file, and the wrapper reports only the account Codex supplies.

The sign-out hook makes the same checks and stops the client's process tree as the retire hook
does. It runs `codex logout -c cli_auth_credentials_store="file"` with the same home, deadline
and failure texts as the retire hook's logout. On success it answers
`client signed out of this Agent's own home` and keeps the home and every entry logout leaves.
A failed sign-out may or may not have removed the login: Codex can fail after removing it, and a
logout stopped at its deadline can stop at any point. The engine reads the outcome as unknown
(IFC-010). The engine holds the instance under PND-011 until an account check finds the expected
account or none.

Order 197 read Codex 0.167.1 at its source tag without running the client. The following behavior
comes from that source. [Order 197's results](../../experiments/027-a5-sign-in/results-2026-11-17-account-read-source.md)

- Before the app-server starts, Codex's bootstrap reads a `.env` in the home. The hook refuses
  such a home. The bootstrap also creates helper aliases under `tmp/` in the home. A release build
  refuses to create them for a home under the system temporary directory and writes a warning to
  stderr, which the hook discards. [R4](../../experiments/027-a5-sign-in/results-2026-11-17-account-read-source.md#r4-what-answering-reads-writes-refreshes-or-calls)
- Startup loads the login from the home's file store into memory and creates and writes state files
  in the home. Codex returns the email it parsed from the ID token's email claim, or null if the
  claim is absent. [R3](../../experiments/027-a5-sign-in/results-2026-11-17-account-read-source.md#r3-where-the-email-comes-from),
  [R6](../../experiments/027-a5-sign-in/results-2026-11-17-account-read-source.md#r6-what-it-returns-when-the-claim-is-absent)
- `refreshToken` false asks for no refresh. Without the block below, answering may refresh a
  ChatGPT login and rewrite the home's login file when configuration acquires auth for a token
  near expiry or a cloud configuration bundle, or when a routing call receives an unauthorized
  response (R2, R4); with it, no refresh completes, and a blocked refresh is not remembered. Startup
  and answering may call OpenAI for a configuration bundle, workspace routing, or analytics at
  initialization. Workspace routing can also be discovered in the background after initialization.
- The account check's child alone receives `CODEX_REFRESH_TOKEN_URL_OVERRIDE` after the wrapper
  applies the launch environment. The endpoint is under `.invalid`, which never resolves
  (RFC 6761, section 6.4). The wrapper passes an allow-list of variables to its clients and
  removes both overrides as a guard if that list grows. Turns in both versions, sign-in's
  `codex login`, and the sign-out and retire hooks' logouts receive neither variable. The check
  also runs without the revoke override, `CODEX_REVOKE_TOKEN_URL_OVERRIDE`. Login and logout
  send stored tokens to the revoke endpoint, and the removals keep a template from choosing
  that destination (ruling 86, point 5).

  At `rust-v0.167.1`, Codex declares the overrides in
  `codex-rs/login/src/auth/manager.rs:232-233`. It reads the refresh endpoint when building
  the auth client at load (`codex-rs/login/src/auth/manager.rs:452`) and at each lookup
  (`codex-rs/login/src/auth/manager.rs:1872-1878,3618`). The revoke lookup reads its own
  override first and then derives the endpoint from the refresh override
  (`codex-rs/login/src/auth/revoke.rs:134-152`). The refresh saves only after the endpoint
  answers (`codex-rs/login/src/auth/manager.rs:3630-3656`). The blocked check completes no
  refresh and saves nothing to the login file. A blocked refresh is not remembered. With an
  expired access token, a 401 from workspace discovery can leave `account/read` answering
  with an error. The hook reports `account hook (account/read): error answer, code N` and fails.
  In the probe the discovery error was a timeout. A turn can still refresh as before.
  [Order 208's results](../../experiments/027-a5-sign-in/results-2026-11-19-refresh-probe.md)

  Where a system or environment proxy is configured, the refresh request can reach it
  (`codex-rs/http-client/src/outbound_proxy.rs:373-403`). A system proxy that receives the request
  can read the token only if it intercepts TLS with a certificate the system trusts, as it could
  for Codex's own refresh. This is an inference from the source outside the probe's
  coverage. Codex records the override's presence in telemetry and feedback
  (`codex-rs/login/src/auth_env_telemetry.rs:15,26,41`), so analytics sent by the check carry
  that flag without a token. The variable is undocumented and used only by Codex's tests.
  It must be read again at every version bump. The source read covers 0.167.1.
- The CLI marks `app-server` experimental. `account/read` and its parameter are unmarked.
  [R5](../../experiments/027-a5-sign-in/results-2026-11-17-account-read-source.md#r5-whether-it-is-marked-experimental)

The hook was built against a scripted stand-in. A5's stage 1, step 1 observes the hook against
Codex and tests the assumed framing of one JSON message per line without a `jsonrpc` member.
The source read covered version 0.167.1 only and did not cover the framing.
[Pre-registered live check](../../experiments/027-a5-sign-in/README.md#stage-1-step-1-the-account-check)

The reported account is the one OpenAI's ID token named at sign-in, as Codex parsed it. The hook
does not check the token itself. A ChatGPT login whose ID token has no email claim leaves sign-in
pending. A machine-managed configuration can set the credential store, and the source read did not
show the hook's `-c` overriding it.

The coordinator expects the app-server to read the home's `config.toml`, which the Agent's own
tools can write. A turn passes `--ignore-user-config`. The app-server's `-c` pins only the
credential-store setting, so a home configuration could select a provider that needs no OpenAI
login and make Codex answer no account, or change the calls made during the read. A5's stage 1,
step 1 uses a fresh home. Each account read, including each sign-in poll, starts a new app-server.

## The .v2 sign-in hook

`codex.v2` states `signIn` with the account place `/account` and two logins. The default
`browser` states `return: redirect`, `carried: true` and an `issuer`. The `device` login
states `return: device`. The hook reads both from the package's `contracts.yaml`, so it
and the Type use the same values.

The literals come from Codex 0.167.1, tag `rust-v0.167.1`.

- Browser client ID: `codex-rs/login/src/auth/manager.rs:1863`, passed to the server in
  `codex-rs/cli/src/login.rs:160-168`.
- Browser issuer and authorization: `codex-rs/login/src/server.rs:78,118,197-207,608-609`.
  The authorization builder adds the parameters once, with `response_type` set to `code`
  and `code_challenge_method` to `S256`. State is 32 random bytes encoded as unpadded
  base64url. The 32-byte SHA-256 challenge uses the same encoding, giving 43 characters.
  `codex-rs/login/src/oauth/authorization.rs:21-48`,
  `codex-rs/login/src/oauth/pkce.rs:14-23`.
- Browser redirect host and path: `codex-rs/login/src/server.rs:197`. The server binds
  IPv4 loopback. It tries the first listed port, sends a cancel request there if it is
  taken and retries, and then tries the listed fallback if the first stays taken.
  `codex-rs/login/src/server.rs:79,81,626-690`.
- Device verification page: the default issuer in `codex-rs/login/src/server.rs:78,118`
  and the path formed in `codex-rs/login/src/device_code_auth.rs:166-176`. The device
  CLI dispatch uses that issuer by default (`codex-rs/cli/src/main.rs:544-553,1661-1667`,
  `codex-rs/cli/src/login.rs:395-406`).
- Device client ID: the constant in `codex-rs/login/src/auth/manager.rs:1863` is the
  default at `codex-rs/cli/src/login.rs:397`, passed to the authorization endpoint by
  `codex-rs/login/src/device_code_auth.rs:63-75,172`.

[Order 195's source read](../../experiments/027-a5-sign-in/results-2026-11-16-0b-source.md)

`codex.v2` states `signIn` only where A5's stage 1, step 1 passed. Order 050n joins this
package into 0.2.0 only on that pass. On a fail, `codex.v2` states no `signIn` (ruling 82,
point 1). [Pre-registered live check](../../experiments/027-a5-sign-in/README.md#stage-1-step-1-the-account-check)

1. The hook refuses a `.v1` Agent, returns an existing configuration refusal when the
   state directory is unset, and refuses once retirement has begun. It refuses a return
   before checking for a retry. A retry reports whether the matching login is waiting
   or has ended and refuses an unknown request state.
2. With no login asked for, or a direct call asking for `browser`, it chooses the browser
   login. It refuses `device` and any other login before checking the home.
3. It checks and if needed creates the Agent's own home, with the same refusals as a turn.
   It reads the stated browser login, stops an earlier login of this Agent if one is
   running, and checks that the client directory still exists.
4. It tries binding the first stated port on IPv4 loopback, then drops the listener.
   If it cannot bind, it starts no client.
5. It starts `codex login -c cli_auth_credentials_store="file"` with `CODEX_HOME` set to
   the Agent's home and the launch environment described above. It ignores stdout and
   waits at most 30 seconds for the first URL on stderr that passes the stated browser
   login's IFC-049 checks. The standard browser URL may also carry the pinned CLI's fixed
   scope, originator and two flow flags. The hook refuses other parameters, including
   configured workspace restrictions and login hints
   (`codex-rs/login/src/server.rs:580-623`, `codex-rs/login/src/auth/default_client.rs:42,64-68`).
   It stops the login group if no URL passes.
6. It gives the URL in an `input_required` result and leaves the login waiting for the
   person. A task discards the remaining stderr, tracks descendants while the login runs,
   and stops its group and known descendants after it exits or after at most ten minutes.
   A failed process-table read makes the scoped stop unconfirmed. Sign-out, retirement,
   a failed or cancelled turn and the wrapper's exit stop a waiting login with the rest
   of the tree. A later retry reads its outcome where the session is still available.

Every failure of the hook to start a login names PND-010. Shared readiness, configuration
and home refusals gain ` (PND-010)` in this hook on a call that starts a login; a retry or a
return keeps their texts.

| Where | Answer |
| --- | --- |
| A URL passed | URL request with request state `agent-codex-login-PID-START` |
| A return came | `sign-in hook (return): PND-013: this hook takes no return yet, and passed nothing to the login` |
| Retry for a running login | `sign-in hook (browser): the browser login is waiting in this Agent's own home` |
| Retry after status 0 and a confirmed stop | `sign-in hook (browser): the browser login ended` |
| Retry after a failed login or a stop with a note | `sign-in hook (browser): the browser login ended: CAUSE` |
| Unknown retry state | `sign-in hook (retry): PND-013: no login of this hook's holds that request state` |
| Device login asked for | `sign-in hook (device): PND-010: the device login cannot start yet: its user code has no path to the engine's panel until F83's code path and order 051c's panel exist; ask for the browser login, or for none` |
| Another login asked for | `sign-in hook: PND-010: the asked-for login is not one that codex.v2 states; it states browser and device` |
| A .v1 Agent, on a call that starts a login | `sign-in hook: a .v1 Agent has none (PND-010)` |
| Retirement has begun, on a call that starts a login | `session retired; implicit restart refused (PND-010)` |
| Retirement has begun, on a retry or a return | `session retired; implicit restart refused` |
| A .v1 Agent or a refused configuration, on a retry or a return | The same text without ` (PND-010)` |
| Refused configuration with no state directory, on a call that starts a login, or a refused home | The shared refusal with ` (PND-010)` appended |
| Stated logins unread | `sign-in hook (start): the stated logins cannot be read (PND-010)` |
| Earlier login's stop has a note | `sign-in hook (stop): NOTE (PND-010)` |
| Client directory gone | `sign-in hook (start): client did not start: the client directory is gone (PND-010)` |
| Port cannot be bound | `sign-in hook (browser): PND-010: the client cannot bind its login port PORT on this machine's loopback: it is in use or not available` |
| Login fails to start | `sign-in hook (start): client did not start (PND-010)` |
| No URL by deadline | `sign-in hook (browser): no login URL within N seconds (PND-010)` |
| Stderr ends without a URL | `sign-in hook (browser): no login URL: client exit: status N (PND-010)`, `sign-in hook (browser): no login URL: client exit: signal N (PND-010)`, `sign-in hook (browser): no login URL: client exit: status unknown (PND-010)`, or `sign-in hook (browser): no login URL: EOF before URL (PND-010)` |
| A stop after a failure has a note | Insert `; NOTE` before the final ` (PND-010)` |

A retry's `CAUSE` gives the exit status or signal, `client exit: status unknown`, or
`no exit within 600 seconds`. It includes any stop note. The texts carry only the failing
step and fixed words and numbers. The URL request is the only result that contains a URL.
Other client output, claims, tokens and file contents stay out of the results and diagnostics.

The source at 0.167.1 shows the CLI printing the URL on stderr and opening the default
browser of the engine user's graphical session where one exists
(`codex-rs/cli/src/login.rs:121-125,170`, `codex-rs/login/src/server.rs:120,208-211`).
Stage 0b found no setting that stops it. The login server binds IPv4 loopback and uses
the cancel, retry and fallback sequence described above. If the home's requirements name
a configuration server, the interactive command signs into that server instead
(`codex-rs/cli/src/login.rs:208-229`). Its URL fails the hook's checks, so the hook fails
with no URL. The home's `config.toml` applies to this login, including changes made by the
Agent's own tools. The `-c` override pins only the credential store.

Before starting the browser login server, Codex attempts to revoke the earlier tokens and
sign the home out. It warns on failure and proceeds with the login
(`codex-rs/cli/src/login.rs:127-142,152,168`, `codex-rs/login/src/auth/manager.rs:1062-1086`).
A hook started after a failed account check under ruling 86, point 3(d), can therefore
leave the home signed out if the person leaves the new login unfinished or Codex's bind
fails because both ports were taken after preflight. A first port already taken at
preflight prevents the hook from starting the client.

The two `.v2` items stay plain in 0.2.0 under ruling 86, point 1. TPL-019 requires the
selected sign-in hook and turn to use the same run item. Keying is for a later release,
so the hook gets no Conduit keys (IFC-007). The engine still relays its URL request inside
`sign-in.v1` (IFC-010, IFC-016, PND-010 and ruling 86, point 3). The engine refuses any
`sign-in.v1` that asks for a login by name, including the device login, before it runs any
part or opens a browser. Its refusal names PND-010. The hook's refusals of a login name or
return guard direct calls. The device login needs F83's code path and order 051c's panel.
Under PND-013 the engine carries no return, so sign-in completes only when the person's
browser can reach the login's loopback port on the engine's machine.

On the engine's own runner, Codex's browser login may open the default browser of the
engine user's graphical session, where one exists. The hook still relays the printed URL
under IFC-049, and the account check (PND-011) stays the only guard.
Order 050n's accounts guide tells the person to close the browser window on the engine's
machine and keep that browser free of an OpenAI session. Under PND-010 the engine
serializes sign-ins with the same fixed ports. Another program using the first port makes
the hook fail, and another program can take the port between the check and Codex's bind.
The build uses a scripted stand-in.
Only A5's stage 1, step 1 observes the hook against Codex. The source read covers 0.167.1.
The fixed scope, originator and flow flags the hook accepts were read at rust-v0.167.1
(`codex-rs/login/src/server.rs:580-623`, `codex-rs/login/src/auth/default_client.rs:42`); a bump
of the Codex version reads them again, since a changed value fails the hook with no login URL.

## A new conversation (both versions)

From 0.2.1, `conversation.v1` includes the operation `new-conversation.v1`, which the wrapper
exposes as `new-conversation`. It takes no input and starts no model work. A Conduit can grant
either operation on its own or grant both.

The call waits for a running turn or hook. It ends the conversation and stops the client as at
a failed turn, including the whole process tree in `.v2`. It forgets the conversation's thread,
failure and turn count. Client launch waits for the next `turn`, which resumes no earlier
conversation.
It launches Codex without `resume`, so Codex starts a new thread.
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

## The login Resource

`codex-login.v1` is a Pool-owned Resource for one Codex account on one machine. Its `account`
is the expected account, on any subscription plan Codex supports. The Resource needs the
engine's own runner and keeps a private `codex-home` in its state directory. Its `sign-in`
hook starts `codex login` there with the file store pinned. Its `account` hook asks `account/read`
with refresh blocked and checks that a reported login lives in a regular `auth.json`. A managed
setting that stores the login elsewhere prevents its Agents from sharing it.

Root sends `sign-in.v1` to the Resource through its owning Pool. A re-sign-in deletes the old
login before the browser step, so a turn in the meantime can report a lapse. Sign-out runs
`codex logout` in the Resource's home. Revocation at the vendor is best effort. Retirement also
empties that home. Retire and unregister the Resource's Agents before retiring the Resource.

## The .v3 Agent

`codex.v3` takes a `login` reference. On the Resource's node, the engine's own runner passes
its state directory in `AGENT_LOGIN_DIRECTORY`. The setup hook and each launch link the Agent's
`client-home/auth.json` to the Resource's `codex-home/auth.json`. Setup starts no client and can
create the link before the target exists. The wrapper refuses an existing file or a link to
another destination without replacing it. The Agent retires by stopping its client and emptying
its home, including the link. The Resource's hooks own sign-in and sign-out.

Agents share the login file, its lock and temp files, and the account's usage limits. Each Agent
keeps its own `config.toml`, history, sessions, SQLite stores and conversation context. Use local
disk. Windows links are unchecked. Each launch disables `use_agent_identity`. Its behavior
needs review at each Codex bump.

Parallel turns rely on Codex's file guards and OpenAI's refresh-token reuse window (DEC-046).
A refresh collision on a 401 path can fail one turn. A stop mid-exchange or sleep past expiry can
end the login for every Agent. A refresh failure that fails no turn goes unseen, so the first
reported failure may already mean that the login is gone. Follow OpenAI's guidance and the live
login's behavior during the shared-login trial. An Agent's own unconfined tools can still log out
the shared login through `CODEX_HOME` (ruling 70).

A dangling link reports a lapse before any client starts and leaves the conversation alone.
A client authentication failure that names the login ends the conversation and reports the lapse
through IFC-016. Usage limits and an engine MCP 401 keep their ordinary failed result. A lapse
raises one unanswered sign-in action and one one-at-a-time notice on the Resource. The report
holds no Agent. The Resource's own sign-in hold, if it has one, holds every Agent's model work.
Root acknowledges the notice through the Resource's Pool to lift the one-at-a-time rule.

### Set up a shared login

Install the 0.2.3 archive and pin it in the project's `conduit.yaml`. Submit a project with
this short-form document, using the expected account in `config.account`.

```yaml
apiVersion: github.com/shrav-k/conduit/v0
fleet: operators
pools:
  logins:
    type: github.com/shrav-k/conduit/core#pool.v1
resources:
  codex-login:
    type: example.org/conduit/agent-codex#codex-login.v1
    owner: logins
    config: {account: operator@example.invalid}
agents:
  codex-worker:
    type: example.org/conduit/agent-codex#codex.v3
    config: {login: codex-login}
    operator: {from: example.org/conduit/agent-codex#turn}
```

The Agent and the Resource use the engine's runner. Leave `runtime` unset and place the Agent
on the Resource's node. With the engine's own machine provider, both default to its local
node. An Agent with a runtime provider or a different node fails setup under PRV-029.

As root, use `logins.set-up.v1` for `Resource/codex-login` and complete sign-in through
`logins.sign-in.v1` before setting up any of its Agents. Then use `operators.set-up.v1` for
`Agent/codex-worker`.

## The .v4 Agent

`codex.v4` uses the same login Resource as `codex.v3`. Its optional `instructions` value
adds developer instructions on every launch, including a resume. `outputSchema` constrains
the final response to a JSON object. Supply the schema as JSON text because MAN-017 binds
scalar configuration to the environment. The schema must be an object whose `type` is
`object`. The wrapper writes it to `output-schema.json` in the run's private directory and
passes it with `--output-schema`.

`instructions` accepts 1 to 21,841 characters. `outputSchema` accepts 1 to 32,762 characters.
These limits fit Linux's per-string exec limit with the environment variable name, up to four
bytes per character, and space for worst-case TOML escaping in the instructions argument.
Either input can be null, which omits its flag. Registration rejects an empty string.
If the output schema fails the object check, every turn returns
`invalid configuration: outputSchema (AGENT_OUTPUT_SCHEMA): expected the JSON text of a schema whose type is object`
and starts no client.

The wrapper returns the final message as `text`. If the text parses as a JSON object, the
result also gives the parsed object in `structured`. Codex's model service enforces the
schema.

## The .v5 Agent and its history

`codex.v5` keeps turns that take the session, including a lapsed login or a refusal after
a failed turn. Configuration refusals, including the home and login-directory checks, and
turns after the retire hook leave no entry. Each entry has an index starting at 1, the
caller's message, the final `text` of a completed turn, and the invocation, caller, state
and UTC timestamps. Attempts of the same invocation have separate indices. `invocation`
and `caller` are null until the engine sends them.

History keeps the message and final text as written. It discards separate tool calls,
tool results, file reads and their contents, and usage limits even when the client stream
carries them. It also discards reasoning, usage counts, failure text, other client output,
and separate credential, environment or configuration values and paths. The engine keeps
none of this history.

The wrapper writes `history.jsonl` beside `client-home` in `CONDUIT_STATE_DIR`, mode 0600.
It records a turn before the client starts and writes its end before replying. A turn whose
start cannot be written fails without starting a client. A failed end write leaves the reply
as it was and the entry reads as `interrupted`. History lasts for the instance's life across
new conversations and runs. History has no size limit and grows with each turn's message
and final text. The retire hook stops the client, removes history and then empties the home.
The engine's own runner removes the state directory with the instance
(PRV-029, LIF-035). For another runtime provider, its contract governs removal. When the
Agent's own tools are enabled, they can change the file (ruling 70). If a change makes the
file invalid, retire the Agent to remove the file and register a replacement.

`history.v1` returns the oldest entries with `index` above `since` (default 0), up to
`count` entries (1 to 50, default 20), and the highest index kept in `latest` (0 for an
empty history). A `running` entry has null `text` and `endedAt`. Read it again from one
below its index. A turn is `completed` when it returns final text and `failed` for any other
reply. An entry whose end was never kept and which no turn serves reads `interrupted`, with
null `endedAt`. Reading works during a turn and starts no client or model work.

## Usage in each turn's result

Completed turns can report `io.github.shrav-k.conduit/usage` in the result's `_meta`. The
engine keeps it in the invocation's end record under IFC-051. For the configured model, the
wrapper reports the client's per-turn input and output counts, including the cached-input
and reasoning counts when available. Codex supplies running totals on resume, so the wrapper
subtracts the last reported totals in the conversation. `new-conversation` starts a fresh
baseline. The client version comes from one bounded `codex --version` probe per run with
reportable usage.

The member is omitted when the model is empty, the version is unreadable, counts are missing,
the input difference is zero, a count falls, or the turn fails.

## A usage limit's window and reset

From `.v2` onward, a `usage-limit` failure can report `windowMinutes` and `resetsAt` in
`failedTurn.limit`. After stopping the failed client, the wrapper asks the client for
`account/rateLimits/read` with refresh blocked. It takes a reached window with the latest
reset, using the client's limit classification if no window is at 100 percent. The read has
at most 15 seconds and uses only the time left in the turn's deadline. A missing answer or field
gives null. `resetsAt` is an RFC 3339 UTC timestamp. Codex supplies no name for `window`, so it
is null. DEC-046 pauses only that account's lanes until the reported reset.

## Model, effort and what stays off

| Configuration | Client argument | Default |
|---|---|---|
| `model` on every version | `--model` | Empty string, using Codex's model default |
| `effort` on `.v2` and later | `-c model_reasoning_effort=...` | Empty string, using Codex's effort default |

Every launch passes `--disable memories` and `-c skills.include_instructions=false` to keep
memories and automatic skill instructions off. A skill named in the message can still load.
The existing flags keep hooks, plugins, multi-agent tools, rules and AGENTS.md discovery off.

## The vendor client

The lowest client version this release was tested with is Codex 0.167.1.
The package's digest covers the wrapper and this bound, not the client.
The wrapper finds `codex` on the PATH of the machine where it runs (PRV-019).
Use the vendor's instructions to install or update the client. [Codex](https://github.com/openai/codex)

Codex `.v3` and `.v4` check the client. The other Types and login Resources do not run the check (TPL-012).
The setup hook runs one `--version` with a five-second deadline and a 256-byte output limit.
Here `P` is the absolute path of the client found and `V` is its reported SemVer version.

```text
client: codex V at P; lowest tested 0.167.1
client: codex V at P is older than 0.167.1, the lowest version this release was tested with; update it by the vendor's own route
client: codex at P gave no version this release can read from --version; the lowest version it was tested with is 0.167.1
client: codex not found on PATH; install 0.167.1 or later by the vendor's own route
```

The last three results fail the hook. The engine names the hook part in its failed setup reply.
A later engine change will include the hook text in that reply. A failed hook runs again at
the next setup. A succeeded hook is kept, so replace an Agent to check a later client (OPS-021,
OPS-028).

## The standard Conduit templates

From 0.2.2, `turn` grants `turn.v1` alone, and `conversation` grants both `turn.v1` and
[`new-conversation.v1`](#a-new-conversation-both-versions). Write
`operator: {from: example.org/conduit/agent-codex#turn}` to grant root turns. Write
`operator: {from: example.org/conduit/agent-codex#conversation}` to let root also end a
conversation. A later release keeps each template's operation list. A wider grant takes a new
template (PKG-031).

From 0.2.6, `history` grants `history.v1` alone for a `codex.v5` Agent. If root only
needs to read history, put `operator: {from: example.org/conduit/agent-codex#history}`
in the Agent definition. If `codex-worker` already has a `turn` or `conversation`
operator, add a separate grant under the document's `conduits` key. Its distinct `as`
name gives root the tool `codex-worker-history.history.v1` (SCT-009, IFC-014).

```yaml
conduits:
  codex-worker-history:
    from: example.org/conduit/agent-codex#history
    consumer: {kind: Interface, name: root}
    target: {kind: Agent, name: codex-worker}
    as: codex-worker-history
```

The 0.2.2 templates left the 0.2.1 Types, homes, and sign-in in place.

## Moving to 0.2.3

Install the 0.2.3 archive, move the pin, and replace each Agent that should use the new release.
An instance keeps the release it resolved. A replaced `.v2` Agent signs in again. To move to
`.v3`, retire each `.v2` Agent first to log out its own login. Confirmed shutdown removes the
old Agent automatically (LIF-035). If it still exists in the retired state, wait for shutdown
confirmation or unregister it before reusing the name (LIF-017). A tombstone can also block the
name (LIF-041). Then sign in each account's login Resource once per machine and register the
`.v3` Agents that refer to it.

## Moving to 0.2.4

Install the 0.2.4 archive, move the pin, and replace each Agent that should use the release.
A `.v4` Agent replacing a `.v3` Agent can use the same `login` Resource without another sign-in.

## Moving to 0.2.5

Install the 0.2.5 archive, move the project pin to its version and digest, and replace each
Agent that should check its client.

## Moving to 0.2.6

Install the 0.2.6 archive and move the project pin to its version and digest. Replace each
Agent that should keep history with a `codex.v5` Agent. The same `login` Resource works
without another sign-in.
