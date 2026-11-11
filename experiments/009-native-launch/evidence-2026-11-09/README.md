# Evidence for experiment 009, 2026-11-09

The raw records described below are [kept at C0](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/009-native-launch/evidence-2026-11-09).
This index preserves their trimming and sanitization method.

These are trimmed records for the [dated results](../results-2026-11-09.md), read offline from
the admin's three external record directories. Directory names here identify attempts only;
no original machine path is retained. Matrix boundaries supplied by the admin were:

| Attempt | Start, UTC | End, UTC |
| --- | --- | --- |
| 1 | 2026-11-09T22:53:14Z | 2026-11-09T22:53:51Z |
| 2 | 2026-11-09T22:54:52Z | 2026-11-09T22:55:24Z |
| 3 | 2026-11-09T22:56:48Z | 2026-11-09T22:59:11Z |

Each attempt has nine JSON files: `claude-r1.json` to `claude-r3.json`, `codex-r1.json` to
`codex-r3.json`, and `pair-r1.json` to `pair-r3.json`. The results link every one.

Each scenario record retains:

- `scenario_status`, from the scenario's `status.json`.
- `stub.records`, with every original stub JSONL row and its `source_line`, including every
  request, instance attribution, validity, response, access change and notification delivery.
  Request `body` strings are decoded to JSON; SSE `data:` JSON is decoded into `messages`.
  Header copies and transport session fields are omitted; no authorization header is copied.
- `clients`, with each client's version and version status, `launch.json`, `settings.json`,
  `mcp.json`, turn plans, prompts, status, start report, final text and available process stderr.
  The generated Codex `mcp.json` is not its effective configuration: its overrides are in the
  recorded client arguments. A turn without a status file is marked `not_started`.
- Selected client `events` with their original JSONL line numbers: startup, tool calls and
  results, assistant text and completion boundaries. Streaming deltas, rate-limit events,
  thinking events, signatures and billing details are omitted, and so is Claude's usage;
  Codex's turn-completion events keep their token counts. Claude message content is projected
  into `content`; Codex item events retain their original structure.
- `wrapper_exchange`, with original `exchange.jsonl` line numbers. The one-tool catalog keeps
  its name and input schema. Successful turn replies keep their text and refer to `turns` for
  events, omitting duplicate structured content. Failure replies and initialization are kept.
- `request_attribution`, a derived index of all request IDs by instance, checked against the
  unabridged request rows. It does not replace them or claim process isolation.

[classification-counts.json](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/009-native-launch/evidence-2026-11-09/classification-counts.json) has 36 rows: one per client and
scenario in each attempt. It uses the order's interpretation of the report schema. The 54
started turns are counted as 42 `told_user` and 12 `other`; the 18 unstarted second turns are
not outcomes. Classification was checked against tool events, stub records and final answers,
not inferred solely from successful wrapper status. The results quote the failure outcomes.

[harness.json](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/009-native-launch/evidence-2026-11-09/harness.json) inventories physical lines and preserves selected source excerpts
with original file and line numbers from the admin-designated order-010 working tree for
attempt 3. It is not the older harness in this checkout and is not a full source snapshot.
It also identifies excerpts of the reused experiment-008 library; that library is excluded
from the 009 line totals. The first two attempts' plans and errors record their differences;
no complete historical source snapshot for those attempts was supplied.

Machine-specific values are replaced with `<HARNESS>`, `<RUN>`, `<RECORDS>`, `<HOME>`, `<USER>`,
`<PATH>`, `<SHELL>`, `<MEMORY>` and `<SOCKET>`. These are placeholders, not executable paths.
Synthetic instance IDs, thread and conversation IDs, loopback URLs, dummy document names and
protocol JSON pointers are retained to check attribution, continuity and observations.
Identity directories, including helper logs within them, were never read or copied. No token,
cryptographic key, certificate or credential value is retained. Authentication *labels*,
environment-variable names and code that refers to identity files are not credential values.
