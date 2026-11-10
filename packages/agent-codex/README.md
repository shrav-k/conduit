# agent-codex

Maintainer: the Conduit project's owner. Native-client launch baseline: Codex 0.167.0, from
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

The last command builds the host wrapper and writes `dist/agent-codex.tar.gz` and its `.sha256`
sidecar. It prints an absolute `ARCHIVE#SHA256` argument for `conduit register --package`.
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

Codex resumes the exact thread in a fresh process per turn. Prompts are written verbatim
to stdin with the `-` prompt argument, including literal `-` and option-like text. `--ignore-user-config`,
`--strict-config` and `--ignore-rules` exclude user configuration/rules;
`-c project_doc_max_bytes=0` also prevents ancestor/project AGENTS.md discovery, which
`--ignore-rules` alone did not stop (finding 3-05). The sandbox is read-only and web search
disabled. Every launch and exact-thread resume disables `shell_tool`, `unified_exec`,
`code_mode_host`, `browser_use`, `browser_use_external`, `in_app_browser`, `computer_use`,
`multi_agent`, `hooks`, `plugins`, `view_image`, `goals` and `image_generation` with
`--disable`. Apps are explicitly disabled unless accountServices is
true, when they are explicitly enabled. Codex has no separate peer-messaging launch switch
in the qualified baseline; peerMessaging is accepted/bound but changes no Codex argument.
The HTTP server is required and its default_tools_approval_mode is approve. Its header helper
receives the identity directory on its command line because Codex removes the run environment
from helpers. No env_vars or inline token is configured for that server.

The client's own tools that reach the machine, a shell, code execution, a browser, another
Agent, hooks and plugins are off by default, alongside account services and messaging.
`accountServices: true` enables `apps` alone; none of the disabled features is turned back on.
The admin's direct trial still listed `apply_patch` (in the read-only sandbox) and a request
for user input. These remain routes the closeout must report and exercise where possible,
along with any other native tool still exposed; their presence is not engine authorization.

**Mediated access (ACC-027):** the former read-only shell bypass is disabled by these launch
arguments. Closeout must ask Codex to read an outside sentinel without engine tools and
observe the result, not infer confinement from absent engine requests. Shared OS identity,
access to other run credentials and the shared login still prevent strong isolation. Any
opted-in app or other remaining native tool is a route around the engine and must be reported.
