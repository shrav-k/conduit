# Construction, interfaces and runtime interaction

Research synthesis, updated 2026-10-31. The accepted contracts live in
[DESIGN.md](../DESIGN.md#establishment-contracts-and-instance-state). The
[concrete YAML packet](../../experiments/001-shared-access/README.md) replaces the earlier partial
Pool/Agent examples. This report preserves the reasoning and source comparisons; it is not
another schema or review queue.

## The boundary that matters

A template is a repeatable recipe. Its graph composes typed plugin operations for establishment
or a supported control. The resulting instance owns ongoing processes, connections and state.
This lets a database start under instance management before a migration step finishes without
turning the graph into a permanent service owner.

| Layer | Responsibility | What it does not establish |
| --- | --- | --- |
| Typed template | Configuration, graph, runtime declaration, dependencies and permitted controls. | A declaration alone does not prove readiness or authorize access. |
| Plugin operation | A narrow typed operation with declared effects, output validity and recovery behavior. | Arbitrary code is not automatically resumable or safe to repeat. |
| Graph | Dependencies and explicit ordering between operations. | It does not own continuing services or replace their runtime contracts. |
| Instance | Resolved definition, confirmed settings, managed behavior and operation evidence. | It does not follow template edits or silently upgrade implementation code. |
| Interface | Unified creation, control and observation across the model. | It is not the only producer of evidence or a requirement for an always-running UI. |

This separates three kinds of change: author a new definition, invoke an existing control, or
change an attachment. A configuration language needs to express each without treating every
YAML field as writable. The [configuration research](CONFIGURATION.md#template-defined-changes-research-synthesis)
connects this distinction to Kubernetes transitions, Docker updates, JSON Schema and Pi RPC.

## Shared service setup

A Pool provisions an authentication Resource and a service Resource. The service declares a
required authentication capability; Pool composition binds it to the concrete owned Resource.
The authentication instance manages sign-in/refresh. Service consumers use authorized Conduits,
so adding an Agent need not repeat provider login or reveal a token. Shared identity does not
require a shared socket, and provider attribution may still show only the shared account.

For MCP, preparation can validate a managed authentication binding, start an instance-owned client
and expose its permitted catalog. The graph can then complete while the client continues. Dynamic
provider tools remain distinct from fixed instance controls. Reconnection, discovery refresh,
permission changes and definition upgrades have different contracts.

MCP [transport](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports) and
[versioning](https://modelcontextprotocol.io/specification/2026-07-28/basic/versioning) require
adapter compatibility checks. A Conduit logical connection is not an MCP wire session, and
transport support alone proves neither retained history nor safe replay.

## Why YAML plus graphs

YAML is a representation of typed data. A graph adds explicit dependencies and operations without
requiring a universal shell language. Finite acyclic graphs keep the v0 language reviewable;
provider-internal branches, challenges and polling stay inside plugins. Templates can use inline
graphs or references, but instances retain their resolved graphs. Setup executes at instance
start; Docker build-time caching is inspiration for future optimization, not the current contract.

Pool composition reuses named entries and their explicit provision/use-existing choices. One
reusable composition operation can establish that selected set and return concrete bindings.
Listing available entries deploys nothing, and no second manually maintained plan is needed.

## Source evidence

The following primary sources were checked in the earlier investigation. Conduit implications
are design inferences, not claims that these systems implement this proposal.

| Evidence | Implication |
| --- | --- |
| [Compose services](https://docs.docker.com/reference/compose-file/services/) and [build configuration](https://docs.docker.com/reference/compose-file/build/) | Keep reusable configuration and actual instance state distinct; inline/referenced recipes can share semantics. Conduit still runs setup at instance start. |
| [compose config](https://docs.docker.com/reference/cli/docker/compose/config/) and [include](https://docs.docker.com/reference/compose-file/include/) | Support resolved inspection and reusable sources with explicit path bases; resolution is not proof of successful effects. |
| [Compose merge rules](https://docs.docker.com/reference/compose-file/merge/) | Explicit replacement/child edits avoid importing surprising field-specific merges into graphs and authority. |
| [Compose startup ordering](https://docs.docker.com/compose/how-tos/startup-order/) | Distinguish readiness from completion, including managed startup before migrations. |
| [BuildKit](https://docs.docker.com/build/buildkit/) | Authoring and execution representations can differ; caching is not an authority or freshness guarantee. |
| [Temporal activities](https://docs.temporal.io/activity-definition) | Lost results and retries need explicit effect/recovery semantics; do not infer exactly-once external effects. |

Multi-object composition may become a convenience authoring bundle across the five families.
It need not be another runtime family or make a Pool own Agents, Interfaces or Conduits. Defer
its syntax until the individual contracts work. Detailed supporting research remains in
[configuration](CONFIGURATION.md), [establishment](ESTABLISHMENT.md), [Resources](RESOURCES.md),
[connections](CONDUITS.md) and the Pi reports.


## Failure cases to retain

- Authentication succeeds but its response is lost: reconcile the retained attempt; do not issue
  another login automatically or claim a token was saved.
- A runtime starts and migration fails: keep the instance managed, report incomplete setup, and
  reuse only valid progress on explicit retry.
- A control response is lost: the last confirmed setting remains recorded, with current state
  uncertain until the handler reconciles it.
- A shared credential expires: show affected consumers without confusing shared-service failure
  with withdrawal of one Agent's access.
- A frontend disconnects: retain operation evidence and supported work independently, then show
  the same outcomes when a frontend reattaches.

The [experiment](../../experiments/001-shared-access/README.md) owns the acceptance sequence.
Filesystem, account and database specifics remain in [establishment](ESTABLISHMENT.md).
