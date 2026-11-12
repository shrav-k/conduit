# SCR core scripted MCP Agent client

Shared apparatus for S4, and for S2 through SCR2; not a client for general use. It never
substitutes for a real client where one is required: [S2 P2](../../013-s2-shared-login/README.md#what-it-must-show)
and [S4 E09 replay](../README.md#runs). It does no model work and emulates no native tools.

Build with `cargo build --release --locked`. A harness supplies its own Agent implementation
item launching a separately built `agent-codex` wrapper with these arguments:

```text
agent-codex --client-program /absolute/path/to/scr-client
```

The [package item](../../../packages/agent-codex/conduit.yaml) leaves this injection out.
The runner supplies the wrapper's private working directory and Agent identity. The wrapper
launches SCR in its Codex JSON protocol mode, sends the script verbatim through stdin, and
reads exactly three flushed JSON events: `thread.started` (without `tools`), an
`item.completed` agent message containing the JSON summary as text, and `turn.completed`.
A first thread is `scr-` plus a UUID v4; resume repeats the wrapper's thread ID. The wrapper
reports `clientTools.status` as `not_reported`. Other launch forms exit 2 with no stdout.
The helper executes once with an empty environment; SCR itself reads no environment variables,
configuration or credentials. It reads stdin and barrier metadata/files and writes only its
trace and barrier handshake. The helper's sole JSON header provides the Agent bearer in memory.
Only literal loopback IP addresses over HTTP are accepted; no DNS, redirects or proxies are
used. HTTPS needs TLS apparatus in the first order that needs isolated engine addresses.

## Script

The message is one JSON object. All fields and all steps are checked before creating a file,
running the helper or opening the session. Unknown fields, versions and steps are invalid.
For example (replace the barrier directory with an existing private absolute directory):

```json
{
  "scr": "v1",
  "id": "cell-01",
  "trace": "cell-01.jsonl",
  "barrierDir": "/absolute/path/to/private-barriers",
  "redact": ["synthetic-output-marker"],
  "callTimeoutMs": 60000,
  "barrierTimeoutMs": 60000,
  "steps": [
    {"step": "discover"},
    {"step": "call", "tool": "effect.write.v1", "arguments": {"entry": 1}, "mode": "keyed", "key": "cell-01-a"},
    {"step": "barrier", "name": "first-held"},
    {"step": "call", "tool": "effect.write.v1", "arguments": {"entry": 1}, "mode": "keyed", "key": "cell-01-a"},
    {"step": "call", "tool": "effect.write.v1", "arguments": {"entry": 1}, "mode": "fresh"},
    {"step": "call", "tool": "effect.write.v1", "arguments": {"entry": 1}, "mode": "unkeyed"}
  ]
}
```

`id` (1–64) and `trace` (1–128) use ASCII letters, digits, dot, underscore and hyphen, starting
with a letter or digit. Trace names are local filenames, created new in the working directory.
`steps` has 1–256 entries. `arguments` defaults to an empty object. `key` must be nonempty and
present exactly for `keyed`; `fresh` creates and records a new `scr-` UUID v4. `unkeyed` sends
no Conduit key or extension declaration. Calls require the latest successful discovery's catalog.
Discovery follows at most 16 pages and refreshes on every scripted `discover`. A call waits
for its own reply. HTTP/transport and JSON-RPC errors are recorded and execution continues;
timeouts, opening/discovery failures and undiscovered names stop execution. InputRequired
responses are recorded without sending another request. Repeats and conflicts are explicit calls.

The two optional timeouts are 1–170000 milliseconds, default 60000. SCR has a 170-second whole
script deadline, within the wrapper's 180-second turn limit. A stopped or invalid script still
completes the turn with a summary. No retry, cancellation notification or session reinitialization
is sent. Startup is rmcp 3.9.0's one session-opening exchange (including discovery/legacy
negotiation as that version requires); the session identifies as `scr-client` at its crate version.

## Barriers

`barrierDir` is required for barriers. It must be an existing absolute directory owned by the
effective user and writable by neither group nor others; symlink directories are rejected.
Names use a lowercase letter/digit followed by up to 63 lowercase letters, digits or hyphens,
and are unique within the script. None of a barrier's three files may exist at validation.

SCR flushes `barrier-reached`, then creates `NAME.reached` empty with mode 0600. The harness
waits for that file and creates `NAME.release`. SCR polls for release until the smaller of its
barrier timeout and script deadline, flushes `barrier-released`, creates `NAME.acknowledged`
(mode 0600), and only then executes the next step. The harness waits for acknowledged to know
SCR passed. Polling intervals do not order events: files and server counts do. Files remain for
inspection and the harness owns cleanup. A late release never passes a timed-out barrier.

## Trace and summary

The trace is JSON Lines, created new with mode 0600 and flushed for each event. Every event has
`seq` (starting at 1, without gaps), `at` (UTC RFC 3339 with milliseconds), `elapsedMs`
(monotonic since process startup), `event`, and zero-based `step` when applicable. Events are:

- `open`: `server` holds negotiated protocol revision, capabilities and server information.
- `request`: scripted `method`, `params`, explicit metadata; discovery also has `page` (zero-based).
  rmcp's automatically stamped protocol/client/progress metadata is visible to the wire tests.
- `reply`: `reply` is `result`, `error` or `transport`; corresponding fields hold the complete
  result, JSON-RPC code/message/data, or transport kind and HTTP status when available.
- `timeout`, `barrier-reached`, `barrier-released`, and `end` with status/cause.

The session advertises its implemented script version as an experimental `scr-client` capability.
Keyed/fresh requests preserve it alongside the Conduit extension; wire tests compare their complete
capabilities with the startup declaration. Startup explicitly uses rmcp’s `Auto` lifecycle: current discovery, with legacy initialization
when the server identifies itself as legacy. The rmcp startup exchange is represented by `open`, rather than script request events.
An interrupted process leaves no `end`. The summary names the trace; S4-H collects it from
the wrapper's working directory. The summary text is one object with `scr`, `id`, `status`
(`completed`, `stopped`, `invalid`), `trace`, `stoppedAt` (zero-based or null), `cause`,
`problem` (field identifier only), and `steps`. Invalid scripts return null id/trace.
Each executed step has `index` and `kind`. Discoveries have sorted `tools` and copied reply
metadata (including an error code on failure); calls
have `tool`, `mode`, `key` or null, stable `label` (`call-INDEX`), `reply`, `isError`,
`outcome`, `reason`, `restriction`, `invocation`, `ruleIds` and `code`; barriers have `name`
and `acknowledged`. Missing reply fields are null (rule IDs default to an empty list).
A call blocked by discovery has `pending` and is never sent. Values are copied, never judged.
`cause` includes `script-deadline` as well as the order's URL/token/open/discover/tool/call/
barrier/trace failure causes. Unexpected fields name their path (for example,
`script.misspelledTimeout` or `steps[0].unexpected`), with synthetic markers redacted and
without echoing the field's value. A trace cannot name any barrier handshake file: filenames
are compared with ASCII case folded, regardless of their directories.

One redactor processes trace values and the summary before serialization, including object keys
and nested JSON strings. It replaces the bearer and optional synthetic `redact` markers (at least
8 characters), in raw and JSON-escaped forms, with `[redacted]`. A number containing a marker
or bearer becomes the string `"[redacted]"` whole. Invalid scripts use only markers meeting the
8-character minimum in their reports. Stdout protocol envelopes are written unchanged; the
summary text they carry is already redacted and remains valid JSON. No stderr is emitted.
No header, helper command, token or redact list is recorded. Trace requests retain idempotency
keys for counting unless the harness explicitly marks one for redaction.

The names are centralized in [src/names.rs](src/names.rs), from `metaPrefix` in
[spec/names.yaml](../../../spec/names.yaml) and the “Conduit's extension” row in
[spec/definitions.md](../../../spec/definitions.md). `tools/set_names.py` does not reach
`experiments/`; changes to those names need an explicit update here.

## Qualification

Run one cargo command at a time:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release -- --nocapture
SCR_AGENT_CODEX=/absolute/path/to/agent-codex cargo test --release -- --nocapture
```

Build the wrapper first in its package with `cargo build --release --locked --bin agent-codex`.
With `SCR_AGENT_CODEX` unset, the wrapper launch test prints its skip and reason; an invalid
path fails. With a path, it tests first/resumed turns through the actual wrapper and its own
header helper over rmcp stdio. All direct tests clear the child's environment, give it a
private HOME and plant a sentinel. The in-process rmcp Streamable HTTP server's authentication
layer records POST bodies as received. Tests check received metadata, errors/status failures,
no retries or cancellations, private traces, redaction, and acknowledged ordering. A closed
empty SSE response exercises loss before a reply. A unit test injects a shorter deadline into
the same executor; no production argv, environment switch or deadline override is exposed.
This is apparatus qualification, not evidence about a real client's behavior.

If agent-codex's argv/protocol changes (including order 050), its launch test must fail until
this client's compatible launch form and the harness item are updated. SCR2 adds task drivers,
pending calls, controlled replay/stale names/serving IDs, retained-output checks and clock sync;
S4-H owns fault scheduling and proxying. Neither is implemented here.
