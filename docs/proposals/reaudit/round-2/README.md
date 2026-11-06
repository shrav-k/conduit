# Re-audit, round 2

Status: in progress, 2026-11-06 (plan-audit action A59). Four cold readers are reading the
repository as it stands at commit `bdacd83`, after consolidation pass 1, the follow-up pass, the
core package and schemas, and the conversion of the behaviors the older text accepted.

| Reader | Sees | Looks for | Report |
| --- | --- | --- | --- |
| 1 | `spec/` only | Promises that duplicate or contradict each other, the definitions, the schemas or the core package | `reader-1.md` |
| 2 | `spec/` only | Terms and document shapes: every term defined once, and the schemas, examples and core package agreeing with the promises | `reader-2.md` |
| 3 | `spec/` only | Anything two conforming engines could disagree on, by walking scenarios end to end | `reader-3.md` |
| 4 | The whole repository, from `README.md` | What an outside reviewer with no context would be misled by or unable to find | `reader-4.md` |

## Known before the round

These points are open and are not to be reported again unless a reader has something new:

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
