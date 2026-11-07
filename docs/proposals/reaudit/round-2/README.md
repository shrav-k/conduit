# Re-audit, round 2

Status: done, 2026-11-06 (plan-audit action A59). Four cold readers read the repository as it
stood at commit `bdacd83`, after consolidation pass 1, the follow-up pass, the core package and
schemas, and the conversion of the behaviors the older text accepted. The three who read only
`spec/` reported 125 findings. Reader 3 marked 7 of them as unsafe, and the coordinator classed
one of reader 1's as unsafe too, which makes 8. The findings were resolved under ruling 37,
without a ruling on each, and every choice is logged. Each decision is the coordinator's and not
the owner's, and the owner can strike any of them. The editors of the closing pass and of the
last pass also started three readers of their own, whose findings are rows of those two logs
and are not among the 125. Round 3 is [next](../round-3/README.md).

Accepted by the owner in [ruling 39](../../ruling-39/README.md), with every other
logged decision of the coordinator and choice of an editor.

| Step | Record |
| --- | --- |
| The decisions on the findings, and on what each later pass left | [DECISIONS.md](DECISIONS.md) |
| The fix pass: promises | [log-promises.md](log-promises.md) |
| The fix pass: core package, schemas and checker | [log-documents.md](log-documents.md) |
| The closing pass, with the choices it made | [log-closing.md](log-closing.md) |
| The last pass, on the fifteen points the closing pass left | [log-tail.md](log-tail.md) |
| The check of what the interface says about the Model Context Protocol | [mcp-citations.md](mcp-citations.md) |
| The fourth reader's findings, outside `spec/` | [log-reader-4-fixes.md](log-reader-4-fixes.md) |

| Reader | Sees | Looks for | Report |
| --- | --- | --- | --- |
| 1 | `spec/` only | Promises that duplicate or contradict each other, the definitions, the schemas or the core package | [reader-1.md](reader-1.md) |
| 2 | `spec/` only | Terms and document shapes: every term defined once, and the schemas, examples and core package agreeing with the promises | [reader-2.md](reader-2.md) |
| 3 | `spec/` only | Anything two conforming engines could disagree on, by walking scenarios end to end | [reader-3.md](reader-3.md) |
| 4 | The whole repository, from `README.md` | What an outside reviewer with no context would be misled by or unable to find | [reader-4.md](reader-4.md) |

## Known before the round

These points were open when the round began, and the readers were told not to report them
again unless they had something new. What became of each is in the decisions and the logs
above; the first is still for the owner:

- How a value is marked as secret, so that a promise can say secrets never appear in
  configuration or records. Left for the owner.
- One `package` key on the wire cannot name two unpublished packages.
- Which reason retrieving a result reports once its target is unregistered.
- Whether an object's default is checked before or after its own properties' defaults apply.
- A pointer that selects nothing in a graph's `outputs` binding.
- What a Requirement's `obliges` requires of an end, and where inspecting shows the
  implementation that applies a requirement.
- The shape of the `settings` input of the operation that reconciles a control.
- Whether a repeat's "same contents" is judged before or after a default is filled in.
- What follows when an implementation restores a runtime its type does not allow.
- What the record of a request holds for the caller when the engine could not establish which
  instance sent it.
- An Operation input with a default may be left out without being marked optional, while the
  general rule says an unmarked input is required.
