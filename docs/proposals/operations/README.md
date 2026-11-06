# Operations, outcomes and revisions — accepted

Status: accepted by the owner on 2026-11-05 (plan-audit ruling 20, action A13). The decisions and
rules are written as promises in the specification:
[operations](../../../spec/conventions/operations.yaml) and
[revisions](../../../spec/conventions/revisions.yaml). Spellings follow the
[ruled vocabulary](../vocabulary/README.md).

This is the second batch of fixes. It covers two shared conventions: operation identity and
outcomes, and revision-checked change. An implementation and an engine written by different
people must agree on these, which is why they are in the specification. How an engine keeps the
promises, for example where it stores a record before dispatching, is its runtime profile.

Defects covered, from the [register](../../LANGUAGE_DEFECTS.md): L1, L9, L10, L51, L52, L56, L57,
L58, L60, L61, L64, L98, L99, L100, L101 and L110.

## Decisions

The owner accepted every recommendation.

| ID | Decision | Options | Accepted | Defects |
| --- | --- | --- | --- | --- |
| OD1 | What the three outcomes mean. | (a) Succeeded: the effect the contract describes occurred. Failed: the implementation confirms it did not occur. Unknown: neither can be confirmed, including any partial effect. (b) Failed means only that the operation did not achieve its purpose; effects may have occurred and the result describes them. | (a). It gives agents one rule: trying again after failed is safe, and after unknown it is not. Preservation test 6. | L10 |
| OD2 | How an invocation leaves the unknown outcome. | (a) A new attempt is allowed only when the implementation's contract declares that it deduplicates by operation identity. Otherwise the outcome stays unknown until an actor holding a grant for it records, with what it checked, that the effect occurred or did not. (b) Inspect the target, then try again, as the text says today. | (a). Option (b) is defect L1: model checking applied the effect twice in eight steps. | L1, L64 |
| OD3 | Who may retrieve a retained result. | (a) Only the original caller. It always learns the outcome of its own invocation; the content is checked against its access at retrieval; a withheld result says so. (b) Any actor that currently holds the permission. | (a). Under (b) a result can reach an actor that never asked for it, and a generic error invites the caller to repeat a completed write. | L52, L60 |
| OD4 | How many counters an object has for checking changes. | (a) One revision per object, for templates and instances alike. (b) A registry version for the record and a separate configuration revision for an instance's settings. | (a). The text uses both terms without saying whether they are one value. | L100, L101 |
| OD5 | A field omitted from a control's input. | (a) It is left unchanged. Defaults are not applied to the inputs of a control. (b) It takes its default, as for any operation input. | (a). Under (b) changing one setting silently resets another. | L99 |

## Rules

These involved no real choice once the decisions were fixed. The owner accepted them as a group.

| ID | Rule | Defects |
| --- | --- | --- |
| OR1 | Every invocation ends with exactly one outcome: succeeded, failed or unknown. A request that was never dispatched has no outcome; it reports why, as one of refused, invalid, conflict or unavailable. | L10 |
| OR2 | The implementation states the outcome in every result. An engine may change a result only to unknown: when no result arrives, when the implementation stops, or when a declared output fails validation. | L9, L10 |
| OR3 | A caller gives each invocation an identity that it has not used before on that target. The target keeps one record per identity. | L51 |
| OR4 | A request carrying an identity the target has already seen is never dispatched again. It returns the state of the existing record: in progress, or its outcome. | L56, L57 |
| OR5 | A request that reuses an identity with different contents is not dispatched and reports conflict. | L58 |
| OR6 | A request that carries no identity, as from an ordinary MCP client, is a new invocation each time. Requests are never treated as repeats because their contents match. | L61 |
| OR7 | The revision advances exactly when the stored definition or configuration changes. It never returns to an earlier value. A change that failed, or whose outcome is unknown, does not advance it. | L100, L110 |
| OR8 | A control's result states the settings it confirmed. The engine stores exactly those, after validating the resulting configuration, and stores nothing if that validation fails. | L98 |
| OR9 | While a control's outcome is unknown, the stored configuration and revision are unchanged, and inspection reports the affected settings as uncertain. | L100 |

## Where each is written

| Decision or rule | Promises |
| --- | --- |
| OD1 | OPS-002, OPS-003, OPS-004 |
| OD2 | OPS-012, OPS-013, OPS-014 |
| OD3 | OPS-015, OPS-016, OPS-017 |
| OD4 | REV-001 |
| OD5 | REV-006 |
| OR1 | OPS-001, OPS-005 |
| OR2 | OPS-006, OPS-007 |
| OR3 | OPS-008 |
| OR4 | OPS-009, OPS-018 |
| OR5 | OPS-010 |
| OR6 | OPS-011 |
| OR7 | REV-003, REV-004, REV-005 |
| OR8 | REV-007, REV-008 |
| OR9 | REV-009 |
| A stale revision is refused | REV-002 |

Model checking in [Experiment 004](../../../experiments/004-stateright-evaluation/README.md)
supports OD2 and OPS-018: the rule sets that match the accepted decision applied the effect at
most once in every reachable state, and the rule set that matches the earlier text did not.

## Not in this batch

- What an engine does when it cannot reach the service that holds grants, whether it holds or
  refuses a request, and how a change in access is ordered against a request already under way:
  the runtime profile.
- Attempt numbering and where a record is stored before dispatch: the runtime profile.
- Graph steps, their states and retry: the graph batch and the runtime profile.
