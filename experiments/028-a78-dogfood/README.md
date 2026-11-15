# 028 — A78: Conduit runs its own orders
Not binding.

This is A78's directory in the [roadmap's register](../../ROADMAP.md#experiments-and-probes).
Stage 1 is orders 1 to 3, which root sends through turns on Agents of the `conduit-core` Fleet
([ruling 70](../../docs/proposals/ruling-70/README.md);
[ruling 61, B12](../../docs/proposals/ruling-61/README.md)). A78's claim, pass conditions and
measures for later orders are not registered here yet.

## The setup as run

The engine was pinned at `38c0b4f`. It holds order 074, and neither 070 nor 077:

```sh
git merge-base --is-ancestor 6591a064 38c0b4f  # 074: succeeds (exit 0)
git merge-base --is-ancestor 1a34ccf9 38c0b4f  # 070: fails (exit 1)
git merge-base --is-ancestor 575a1bcd 38c0b4f   # 077: fails (exit 1)
```

The `.v2` Agent packages were installed from `wo/076-agents-v2-folder-grant`; they stay unmerged until order 050.
[Order 076's Record](../../docs/work-orders/076-agents-v2-folder-grant.md#record) says where that branch stands; the inputs give no commit for the installed archives.
The [Fleet templates](inputs/fleet/templates.yaml), [instances](inputs/fleet/instances.yaml) and [harness](inputs/harness.md) give `worker-1` a 4-hour deadline and `verifier-1` 1 hour.
Both are `codex.v2`, with effort `high`, account services off and their own tools on, working
in the same slot clone `../slot-1`; setup fills `@SLOT_1@` with the slot's absolute path.
The tree Pool is empty in stage 1, so no tree Resource is granted; in v0 nothing confines the
Agents' own tools ([harness](inputs/harness.md)).

The coordinator's placeholders (`<...>`) and the Fleet templates' own placeholders (`@...@`):

| Placeholder | Stands for | Class | Input files using it |
| --- | --- | --- | --- |
| `<dogfood>` | Harness directory outside the repository | path | `harness.md`, `plan.md` |
| `<main checkout>` | Main checkout, also a source for a locally held branch | path | `harness.md` |
| `<slot-1>` | Slot clone fetched by the admin | path | `harness.md` |
| `<github url>` | Repository clone URL | other | `plan.md` |
| `@SLOT_1@` | Slot's absolute path, filled by setup | path | `fleet/instances.yaml` |
| `@WORKER_DEADLINE@` | Worker deadline; harness says 4 hours | other | `fleet/templates.yaml` |
| `@VERIFIER_DEADLINE@` | Verifier deadline; harness says 1 hour | other | `fleet/templates.yaml` |
| `@CODEX_VERSION@` | Installed agent-codex version; value not stated in manifest | other | `fleet/conduit.yaml` |
| `@CODEX_DIGEST@` | Installed agent-codex archive digest; value not stated | other | `fleet/conduit.yaml` |

Other text in angle brackets in `plan.md`, such as `<schema>`, `<cause>`, `<turnDeadlineSeconds>`
and `<turnDeadlineSeconds×1000>`, is the plan's own notation, not a redaction.

## The harness as run

The [harness README](inputs/harness.md#what-conduit-ordersh-does) gives the exact commands;
its sequence is:

1. Lock the slot, check it is clean with pushes disabled, fetch the order, create its branch and clean the slot, retaining build output.
2. Before each fresh conversation, stop serve, export records, restart with its call timeout, wait for root's turn tools and check startup setup failures.
3. Send run-order.sh's prompt, plus a paragraph on how the turn runs, through root's turn tool for that Agent (`turn-worker-1` or `turn-verifier-1`), in the background, with the watchdog and a check that serve still runs.
4. Extract the report text, strip a code fence and validate against its schema; allow one repair turn in the same conversation, without restarting.
5. Run guards after every turn, including repairs: HEAD and branch unmoved, verifier tree unchanged, nothing pushed, main checkout untouched; checkpoint worker output, then run the verifier and further rounds as its findings require.
6. On exit, stop serve, export final records and append the run's line to `measures.jsonl`.

## The plan

[The plan](inputs/plan.md) is what [ruling 74's record](../../docs/proposals/ruling-74/README.md)
cites as “The coordinator's plan for A78 under ruling 70”. It plans “one order at a time” on
fixed slots, with root dispatching orders 1 to 3 and a coordinator Agent dispatching orders
4 and 5 after appointment. It plans the Agent packages, the session idle-limit fix, smoke
checks, guards, reports, friction logging and later follow-ups; its earlier order numbers
and own-tools switch remain its words, rather than a description of the final setup.

## The export

The coordinator made [the export](inputs/records-2026-11-12.jsonl) on 2026-11-15 with the
dogfood's pinned engine `38c0b4f`, using `conduit records --store` with serve stopped, before
order 050 creates a new store. By the coordinator's account stderr was empty, the file is
byte-identical to the harness's final export, and it holds no absolute path, user name or
credential. Its filename takes the store's last-write date. From the repository root:

- 344 CloudEvents lines: `wc -l < experiments/028-a78-dogfood/inputs/records-2026-11-12.jsonl`.
- 60 access decisions, 60 dispatches, 60 invocation endings, 104 object changes and 60 request endings: `jq -r .type experiments/028-a78-dogfood/inputs/records-2026-11-12.jsonl | sort | uniq -c`.

## Files

Each input keeps the words it was written with before it was committed. The Fleet files are templates, not rendered copies.
The raw export leaves at the next condensation; the results then link its last version under [keeping evidence](../README.md#keeping-evidence).

| File | Holds |
| --- | --- |
| [README.md](README.md) | Method as run and its limits |
| [results-2026-11-12.md](results-2026-11-12.md) | Dated observation of order 042 |
| [inputs/records-2026-11-12.jsonl](inputs/records-2026-11-12.jsonl) | Engine's raw records; IDs, operations, times, attempts, outcomes and access decisions, without request inputs |
| [inputs/measures.jsonl](inputs/measures.jsonl) | Harness's run measures |
| [inputs/fleet/conduit.yaml](inputs/fleet/conduit.yaml) | Fleet project's manifest template |
| [inputs/fleet/instances.yaml](inputs/fleet/instances.yaml) | Fleet, Agents, Pool and Conduit instances |
| [inputs/fleet/templates.yaml](inputs/fleet/templates.yaml) | Agent, Pool and Conduit templates |
| [inputs/harness.md](inputs/harness.md) | Harness README; code stays outside the tree |
| [inputs/plan.md](inputs/plan.md) | Coordinator's plan cited by ruling 74 |

## What it cannot show

The records say which requests were made, by whom, and how each ended. They say no more.
What an Agent did in its slot with its own tools is not a Conduit request, so it is not in
the records (CTR-014, PRV-006, ACC-027). In stage 1 every caller is root, so the records do
not test attribution among callers. An order run through Conduit is not evidence about
access unless the results count a refusal, an open or a close. The reports of 042's worker
and verifier are not in this directory.
