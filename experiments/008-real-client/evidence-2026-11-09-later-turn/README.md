# Trimmed evidence for the later-turn follow-up

The raw records described below are [kept at C0](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn).
This index preserves their trimming and sanitization method.

These excerpts support the [dated results](../results-2026-11-09-later-turn.md).
There are 12 run directories, each with `context.json`, `turn-1.jsonl` and `turn-2.jsonl`.
The results abbreviate `claude` as `CC` and `codex` as `CX`; every run uses cache off,
`ttlMs` 0 and valid authentication.

## Reading the records

Every JSONL entry identifies its original `source_file` and one-based `source_line`.
Results citations use **W** for `stub.jsonl`, **E** for `events.jsonl`, **X** for
`client-exposed.jsonl` and **F** for `final.txt`, within the stated turn. These are source
line numbers, not physical lines in these combined excerpts. JSON whitespace is normalized.

- **W** retains every request, response status, negotiation, turn marker, list change,
  notification delivery and stream closure. Wire chunks containing JSON are decoded into
  `messages`; SSE keepalives are omitted. Request bodies are decoded into `body`, with
  diagnostic `_meta` omitted except protocol declarations and Codex model/version/effort.
  Tool-list definitions retain their names only; cache hints, call results and notification
  messages remain. Headers and transport session identifiers are omitted. Thus absence of
  a `tools/list` or `tools/call` request is checkable from the complete request sequence.
- **E** retains Claude's initialization tools/model/version, assistant tool calls and text,
  user tool results and result wrappers; Codex's thread/turn boundaries and started/completed
  items. Thinking, signatures, streaming deltas, usage and unrelated diagnostics are omitted.
- **X** retains user tool results or MCP call items from the harness's client-exposed subset,
  with its own source line numbers. This is corroboration, not a model-input capture.
- **F** retains the complete final text as `text`, including its original line breaks.
  `source_line: 1` locates the beginning of that text.
- **context.json** retains the installed version, cell, whole-run status (including both
  turns), and each turn's original fixed prompt and sanitized launch plan. These name their
  sources; `cell` and `status` come from the run-root files of those names.

The client's conversation identifier is replaced consistently with `<session>` within each
run, after checking that the original identifier agreed across both turns. It is distinct
from the omitted MCP transport session identifiers. `<runs>` replaces the original records
root; `<lab>` replaces other absolute filesystem paths. No identity file, credential file,
helper output or authorization header was read or copied. The public protocol namespace
in Conduit's metadata is retained as experimental content, as in the first results' evidence.

Omission of a field does not establish its absence in the source. Client event catalogs and
MCP results do not expose the complete model request. The results distinguish observable
assistant calls and reports from a claim about the exact model input.
