# agent-claude-code

Maintainer: the Conduit project's owner. Native-client launch baseline: Claude Code 2.1.318, from
experiment 009. Work orders 023 and 024 tested this package with its scripted client only; the real
client closeout is still required. No client version command or model was run for this order.

This standalone Rust crate has no dependency on another repository directory. Its lockfile
and pinned toolchain travel with it. Its manifest exports one Agent Type, a conversation
capability and the explicit `turn.v1` operation. The plain MCP wrapper exposes only `turn`;
the manifest maps the versioned operation to it. Both reach inputs are required booleans.
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
`CONDUIT_IDENTITY_DIR`. The manifest binds all three Type inputs through JSON Pointers to
`AGENT_ACCOUNT_SERVICES`, `AGENT_PEER_MESSAGING` and `AGENT_MODEL`. The wrapper inherits only
HOME, LOGNAME, PATH, SHELL, TERM and USER, plus the run's two identity variables, into its
client. It removes ANTHROPIC_API_KEY and OPENAI_API_KEY. The owner's subscription login is
shared; a separate login home is future work. Only the helper mode reads `token`, printing
one JSON Authorization header and nothing else. The token is never placed in environment
variables, arguments or logs. The wrapper retains no prompt, raw event stream or stderr log.
A failed turn poisons the conversation; no automatic restart or repeated model work occurs.
The per-turn deadline is 180 seconds, after which the client ends and the turn fails.
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

Claude Code runs one streamed process for the wrapper's lifetime. Settings and MCP files are
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

The client's own tools that reach the machine, a shell, code execution, a browser, another
Agent, hooks and plugins are off by default, alongside account services and messaging.
With both reach inputs false, no built-in machine or peer tool is selected; engine MCP tools
remain. Account connectors and the three named peer tools are explicit opt-in routes around
the engine. Closeout must record any other native tool the client still exposes and test its
reach. Shared login and OS-user access to files and run credentials remain mediated limits;
these launch settings do not provide OS isolation.
