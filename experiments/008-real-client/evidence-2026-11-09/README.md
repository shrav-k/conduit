# Trimmed evidence for experiment 008

The raw records described below are [kept at C0](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09).
This index preserves their trimming and sanitization method.

These are sanitized excerpts of the admin's 68 matrix records, prepared for the
[dated results](../results-2026-11-09.md). The full directory name identifies each run; the
results abbreviate `claude` as `CC` and `codex` as `CX`. The five records in
[smoke-checks.jsonl](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09/smoke-checks.jsonl) are separate harness checks, not matrix runs.

## Reading an excerpt

`source_line` is the original file's one-based line number. JSON whitespace is normalized;
remaining field values are copied, except for path substitution. It is not an event number
invented by the classifier. Missing fields or lines were omitted, not observed to be absent
from the original. In particular, no model-input claim follows from trimming a client event.

| File in each run directory | What is retained |
| --- | --- |
| `stub.jsonl` | Every HTTP request, every response status, negotiation, list changes and delivery records, and wire chunks containing JSON. SSE keepalives are omitted. Request headers retain protocol revision/method/name only; response headers retain the 401 challenge only. Session identifiers are omitted. Request-body `_meta` retains protocol/client declarations and Codex's model/version/effort fields; other diagnostic fields are omitted. Inside wire `data`, each tool definition is projected to its unchanged `name`; list cache hints and the rest of the wire message remain. Error and call-result messages are retained in full. |
| `events.jsonl` | Claude's initialization fields for version/model/tools/authentication, assistant and user conversation messages, tool-result logging, and tool-name mappings; Codex's started/completed items. Thinking blocks, signatures, duplicate streaming deltas, usage accounting, unrelated startup fields and duplicate final-result wrappers are omitted. |
| `client-exposed.jsonl` | The corresponding Claude user/tool-result events or Codex MCP call items from the harness's client-exposed selection, with their own original line numbers. Assistant messages are retained in `events.jsonl`, avoiding another copy here. An empty file means there was no selected tool-result event, not that no model message existed. |
| `final.txt` | Complete final text, with original line breaks. Empty for the excluded initialization failure. |
| `version.txt`, `status.json`, `cell.json`, `prompt.txt` | Installed version, runner status, requested variant/repetition and fixed prompt. |
| `plan.json`, `mcp.json` | Launch configuration and helper command with sanitized paths, including cache switches and Codex's server approval. These contain credential file locations, never their contents. |
| `helper.jsonl`, `authentication.json` | Helper-read times and the harness's client-reported model-service authentication observations; no helper header output. |
| `stderr.txt` | Present only for the initialization failure; includes the authentication error. |

The complete request sequence is preserved, making the reported absence of a refresh or MCP
call checkable. `events.jsonl` and `client-exposed.jsonl` are client-exposed evidence, not full
model-input captures. The results state explicitly where the observation stops at that
boundary and where a model merely reports an internal attempt.

## Sanitization

The original records root becomes `<runs>`; other absolute filesystem paths become `<lab>`.
Paths below those placeholders keep relative names where present. No identity files, helper
stdout, login files, tokens, private keys or authorization header values were read or copied.
The protocol's Conduit metadata namespace is retained because it is the subject of the
experiment, not a credential. Synthetic invocation identifiers are also retained to distinguish
an outcome from a refusal without an invocation.

The smoke inventory copies only the relative record identifier, cell, status and any Codex
server-approval argument. Its five repetition records cover three client/case checks, as the
results' deviations explain. No smoke record contributes to a model-behavior classification.
