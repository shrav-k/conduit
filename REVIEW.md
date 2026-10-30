# Collaborator review

Updated: 2026-10-30. Read [SPEC.md](SPEC.md) for the current product definition and
[DESIGN.md](DESIGN.md) for contracts. This file contains the remaining choices, not
another decision ledger. Recommendations below are proposals.

## Review sequence

### 1. Confirm the concept model

Check whether the definitions make an actual workflow easy to explain: a role
configures a session; the session selects scopes; scopes grant operations on resources;
a project groups related resources; memory retains knowledge; rooms carry messages.

Role limits and scope grants serve different purposes. Memory, project, and resource
are related but not interchangeable. Working context means information an agent is
using; an owning context means a record's task/project/personal ownership anchor.
Raise a concrete example if any distinction should change rather than creating new
names for the same responsibility.

### 2. Bound the first end-to-end workflow

Use Claude and Codex on the same machine, one project/repository, one room, and useful
shared knowledge. A proposed demonstration is: Claude investigates a change, saves
its finding, and asks for review; Codex retrieves the finding/evidence and replies.
Then demonstrate checkpoint-based continuation with visible gaps if capture fails.

The [first interaction surface](SPEC.md#configure-and-reuse-roles) is settled:
Conduit's CLI launches native agent interfaces. Resolve launch configuration and
account handling, then the minimal memory backend. Choose exact memory read/write/search
operations and their returned evidence before selecting a graph, vector engine,
ontology, or broad plugin schema. Pi and own chat follow the initial workflow.

### 3. Bind collaboration to resources

Resolve these connected questions for that workflow:

| Boundary | What remains to decide |
| --- | --- |
| Role and connection setup | Configuration precedence, credential references/concurrent logins, skill import/update behavior, and capability checks when switching runtimes. |
| Managed access | Which operations/selectors/sharing modes v0 supplies; what chosen integrations can actually restrict; initial observation coverage and MCP routing. |
| Room and work | Membership and visibility, addressing, relation to task/workspace/issue, and the transition to mandatory tracker linkage. Start without a tracker for room testing; Linear remains the first integration. |
| Memory retrieval | Search/read/save contract, relevance and context budgets, source freshness, and handling partial results. Available content must remain distinct from automatically loaded context. |

Preserve native provider operations. The shared layer should not require an internal
ticket engine, replacement skill language, or generic wrapper around every provider tool.

### 4. Agree on acceptance and recovery

Choose the acceptance cases from the specification that belong in the first release.
Set room read-position initialization/advancement, message ordering and retry identity,
acknowledgement meaning, restart recovery, and retention. Resolve workspace concurrent
editing and cleanup. Specify how any selected tracker exchange works; a room post
must not silently become an external comment.

Define enough usage, delivery, and context evidence to explain the demonstration.
Verify host/version support with the chosen adapters rather than claiming universal
coverage. These are design and validation tasks; implementation begins after the
workflow and its required contracts are agreed.

## Keep later work out of the critical path

Remote rooms, role-specific wake hooks, comprehensive MCP enforcement, broader sandbox
tiers, advanced sharing delivery, and automatic classification/extraction stay staged.
Their principles remain in the specification/design; tier names, sandbox technology,
full event schemas, exact costs, and plugin ABI details do not need decisions now.

## Editing the specification

Resolve one coherent boundary at a time. Edit the relevant contract in place and
remove answered questions here. Ask before changing an ambiguous substantive
requirement. Use Git history for previous versions; do not recreate a numbered
chronology or duplicate the specification in a separate decision register.
