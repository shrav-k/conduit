# Experiments

Experiments live outside `docs/`. Each directory keeps its documentation, configuration examples,
expected outcomes and validation record together. The [specification](../docs/SPEC.md) and
[design contracts](../docs/DESIGN.md) remain authoritative for accepted behavior.

| Experiment | Purpose | Current evidence |
| --- | --- | --- |
| [001 — shared access](001-shared-access/README.md) | Exercise all five families, Pool composition, shared authentication, communication, controls and Interface observability. | Documentation and static configuration checks only. Fixture plugins do not exist yet. |
| [002 — runtime and plugins](002-runtime-plugins/README.md) | Test authored/code-optional plugins, graph-backed operations, composed/custom requirements, independent MCP access and effect recovery. | One-machine/two-executor layout and combined authoring proof scope accepted; implementation details remain proposed. No code or runtime measurements yet. |

Distinguish draft syntax, parser/static checks, synthetic runtime tests and live provider tests.
Do not promote expected outcomes into completed evidence. Changes to accepted behavior discovered
here return to review with a concrete counterexample. New experiment directories should follow the
same pattern rather than adding competing rule sets to research reports.
