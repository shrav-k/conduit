# Ruling 71: a faster pace: more coordinators, batch 2 begun now, the bar's rule fixed, small findings delegated

Status: in force (ruled by the owner on 2026-11-11). Holds now: [IN_FORCE.md: 9. Fleets, Interfaces, pending actions, sign-in, and what Agents reach](../../../IN_FORCE.md#9-fleets-interfaces-pending-actions-sign-in-and-what-agents-reach), [IN_FORCE.md: 13. Proof, the build, experiments and measurement](../../../IN_FORCE.md#13-proof-the-build-experiments-and-measurement), [IN_FORCE.md: 15. How the work is governed](../../../IN_FORCE.md#15-how-the-work-is-governed). This record says why.

The owner found an estimate of three to six weeks to v0 unacceptable, and offered more Codex credits
and their own time. The coordinator measured the pace and planned the remaining work as parallel
lanes. The plan put four decisions to the owner, each with a recommendation:

1. More than one coordinator session, with one merge authority. This saves about a week.
2. Batch 2 drafted now on a branch, and merged only after the owner judges S1's bar. This saves
   two to four days.
3. The bar's rule for each task, fixed before anyone sees a count.
4. A stated class of small findings that add a feature, delegated to the coordinator.

The coordinator also recommended leaving git history as it is after removing local paths from the
tree. The owner: "I accept your recommendations, if credits run out inform me and I will do my best
to remedy the situation".

## The decisions

1. **Two or three coordinators, one merge authority.** The admin session of ruling 48 stays the
   merge authority for the engine's core, the conformance runner and manifest, `spec/`,
   IN_FORCE.md and the ledger. One or two more coordinator sessions each run a lane, as the plan
   proposed: scenarios and providers, and proof (safety vectors, model checking, probes).
   - A lane coordinator writes and dispatches its lane's orders.
   - It merges only pull requests confined to its lane's paths (`packages/`, `experiments/`, a
     provider's crate), once CI is green and a fresh verifier is clean.
   - A conflict between lanes goes to the merge authority, and a behavior question to the owner.
   - Every coordinator logs its decisions for the owner to strike, as before.
2. **Batch 2 begins now, on a branch.** Its plan, its specification and its core are written on a
   `batch-2` branch, from the build that the S1 rerun pins. It merges to `main` only after the owner
   has judged S1's bar. The rerun therefore still measures a build without Fleet authority, as
   ruling 61 B1 intended. A second rerun, if one is needed, runs on a branch from the same pin.
3. **The bar's rule for each task**, fixed before any count of the rerun is seen (ruling 45):
   - the measure is the author's lines as ruling 63 counts them, with ruling 68 R1's actions at the
     panel listed apart;
   - both runs must pass: the coordinator's and the fresh Codex author's (ruling 48, point 4);
   - "much more" on a task means more than one and a half times the baseline's count for that task.

   Conduit passes when, in each run, it takes fewer lines than the baseline on most of S1's tasks
   and much more on none (ruling 47).
4. **Small findings that add a feature are delegated.** The coordinator may settle a small finding
   that adds a feature, and logs it for the owner to strike. Until the S1 rerun, this excludes
   anything that could move S1's author counts: CLI flags, short-form keys, templates and defaults.
5. **Git history stays as it is.** Local paths found in five committed proposal files were removed
   from the tree on 2026-11-11. Their earlier versions stay in history.
6. **Credits.** If Codex credits run out, the coordinator tells the owner at once.

## What it amends

- Ruling 48: the single admin session becomes one merge authority among two or three
  coordinators.
- Ruling 61, B1: batch 2 is written and built on a branch from now on, and merged after the bar.
- Ruling 47: the bar's "much more" is defined, and both runs must pass.
- IN_FORCE.md topic 15, "who decides": the owner keeps every finding that adds a feature except
  the delegated small ones.
