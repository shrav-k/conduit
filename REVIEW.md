# Collaborator review

Updated: 2026-10-30. [SPEC.md](SPEC.md) defines the product; [DESIGN.md](DESIGN.md)
records the contracts and their rationale. This agenda contains unresolved choices.
Work through one coherent boundary at a time and update those documents in place.

## Agreed starting point

V0 centers on two Codex and two Claude subscription accounts running concurrently
through Conduit's CLI and native agent interfaces, sharing one room and permitted
knowledge. Account state stays separate; private native context belongs to sessions.
Native execution is the default, isolated execution is available explicitly, and roles
may require it. The [account/isolation contract](DESIGN.md#account-separation-and-execution-isolation)
and [rationale](DESIGN.md#why-both-modes-with-native-as-the-default) cover the agreed guarantees.

YAML authoring, explicit native-source selection, and project scope defaults are settled
in the [configuration contract](DESIGN.md#configuration). Native runtimes own subscription
authentication; Conduit selects accounts and verifies identity. Native tooling handles
skill/plugin installation and updates through adapters. Runtime-specific loading,
credential concurrency, and private-state separation still need experiment evidence.

## Remaining behavioral review

These three groups contain the remaining behavioral decisions for the first workflow.
The existing contracts remain the starting point; resolve concrete gaps rather than
reopening settled definitions or adding speculative features.

| Group | Decisions still needed |
| --- | --- |
| **1. Rooms and workspaces** | The relation among room, task, workspace, and tracker issue. Choose concurrent file-editing and workspace-cleanup behavior. Set the transition to mandatory tracker linkage and any initial Linear exchange; room posts must not automatically become external comments. |
| **2. Shared memory and managed access** | The smallest useful save/read/search contract and its evidence; which selectors, operations, and sharing modes v0 supplies. Define retrieval limits, source freshness, and partial/unavailable results. Available information remains distinct from automatically loaded context; choose backend candidates against this behavior later. |
| **3. Recovery and observable evidence** | Message order, cursor initialization/advancement, acknowledgements, retry identity/deduplication, and restart behavior. Set retention for history, checkpoints, and observations; define the usage/account/context evidence needed to explain the experiment, including gaps and unknown outcomes. |

Keep local isolation, network access, provider-operation permissions, and Conduit
record access distinct. A useful first implementation can have different coverage
at each boundary, provided that coverage is explicit and required restrictions hold.

The behavioral review is complete when each group supplies a concrete v0 contract
or an explicit deferral compatible with the four-account scenario. Unverified adapter
behavior becomes a named experiment check, not an assumed guarantee. Detailed wire
schemas, storage engines, and every possible failure mode need not be decided here.

## Next: first YAML design

After the behavioral review, discuss concrete examples together before freezing a
schema or writing implementation code:

- Choose the configuration schema-version convention, object shapes, names/references,
  file layout, validation, path/endpoint matching, and merge/override rules. Keep configuration versions distinct
  from MCP protocol and plugin versions.
- Express the four roles, account references, shared connection definitions, scopes,
  project, and execution policy. Show the resolved launch settings and native adapter
  output so precedence is reviewable. Keep credentials out of shareable configuration.
- Make execution mode explicit in generated role files as a proposed authoring rule;
  decide whether handwritten files may omit it and use the native default.
- Check both ordinary use and a few conflicting/incomplete configurations. Do not
  require Kubernetes machinery, a custom skills language, or an internal ticket engine.

## Then: initial experiment design

Review the experiment in detail after the first YAML examples. Select supported host
platform/runtime versions, account enrollment, the minimal room/memory backend, and
an isolation backend to evaluate. These are choices for the experiment, not universal
plugin requirements. Derive the needed file/service grants and native capabilities,
then verify that the backend enforces the agreed execution policy.

Define setup, expected observations, failure criteria, restart/cleanup, and the order
of trials. Cover four simultaneous identities, room catch-up, permitted/shared and
private knowledge, independent account lifecycle, visible execution modes, and honest
usage coverage. Specify isolated-mode negative checks and compatibility gaps. Decide
which checks gate the first usable version; distinguish a native-mode demonstration
from evidence that isolation works. Checkpoint handoff remains a separate scenario.

Implementation follows agreement on the behavioral contracts, YAML examples, and
experiment plan. No account registration or live agent experiment is implied by this
documentation review.

## Deferred scope

Remote rooms, role-specific wake hooks, comprehensive MCP operation inspection and
enforcement, advanced sharing delivery, and automatic classification/extraction stay
staged. Pi and Conduit's own chat follow the initial native-interface workflow. Exact
cost accounting, plugin ABI, crate layout, and stronger isolation backends can follow
the evidence from the initial experiment.

Keep accepted reasoning beside its contract in DESIGN.md. Remove resolved questions
from this agenda and use Git history for previous versions; do not recreate a separate
decision chronology.
