# agent-codex

Maintainer: the Conduit project's owner. Native-client launch baseline: Codex 0.167.0, from
experiment 009. Work orders 023 and 024 tested this package with its scripted client only;
the admin's [real-client closeout](../../docs/proposals/first-build/increment-3/closeout-results-2026-11-10.md)
and [increment 4 log](../../docs/proposals/first-build/log-increment-4.md) record subsequent trials on 2026-11-10.

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
access to other run credentials and the shared login still prevent strong isolation. Any
opted-in app or other remaining native tool must have its reach stated; the collaboration
tools below stay within the Agent's own access.

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
still not found; the [log](../../docs/proposals/first-build/log-increment-4.md) records the
trial and settlement.

## The .v2 Agent

The package also exports codex.v2, implemented by the separate `wrapper-v2` item.
A template may select `wrapper-v2-path` instead, as [its section](#the-wrapper-v2-path-item) says.
All properties are required after SCH-009 fills their defaults; authors may omit them.
A null `directory` gives no environment variable (PRV-007).

| Input | Default / bounds | Wrapper variable |
| --- | --- | --- |
| `directory` | `null`; otherwise an absolute path to an existing directory | `AGENT_DIRECTORY` |
| `turnDeadlineSeconds` | `180`; integer from 1 through 604800 | `AGENT_TURN_DEADLINE_SECONDS` |
| `effort` | `''`; otherwise lowercase ASCII letters | `AGENT_EFFORT` |
| `accountServices` | `false` | `AGENT_ACCOUNT_SERVICES` |
| `model` | `''` | `AGENT_MODEL` |


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
Then `-c mcp_servers.engine.tool_timeout_sec=N` equals turnDeadlineSeconds, followed
by optional `-c model_reasoning_effort=E` with E JSON-quoted, then the resumed thread
if any, then `-`. Codex writes no settings.json or mcp.json. Its client environment
is unchanged from `.v1`.


`conduit serve --call-timeout-seconds` must be longer than `turnDeadlineSeconds`.
With their defaults, 120 and 180, the engine reports a turn longer than 120 seconds
before the wrapper's deadline. Longer turns may also require the engine fix in order 074.

## The wrapper-v2-path item

`wrapper-v2-path` is `wrapper-v2` with `command: [agent-codex]`. It binds the same inputs
to the same variables, so everything this README says of `.v2` holds for it. The default
of `codex.v2` is still `wrapper-v2`.

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
`implementation: {example.org/conduit/agent-codex#turn.v1: example.org/conduit/agent-codex#wrapper-v2-path}`
beside its `type` and `config`. An instance of it names its runtime provider in `runtime`
(PRV-006). An instance that names none runs on the engine's own runner, on the engine's
machine (PRV-019).

`directory` is then a path on that machine (PLC-020), and the wrapper checks it there.

The paragraph above on what nothing confines describes the engine's own runner. Under
another runtime provider, the client and its own tools run on that machine, as the
operating-system user that the provider runs them as, with that user's files and whatever
client login that user has there. That provider's contract states what, if anything,
confines them. This package copies no login.

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
