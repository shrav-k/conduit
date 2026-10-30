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

Use two Codex and two Claude subscription accounts concurrently on the same machine,
one project/repository, one room, and useful shared knowledge. Exercise different
session scopes, separate private native state, account restart/logout independence,
and four-way communication. Then demonstrate checkpoint-based continuation with
visible gaps if capture fails.

The [first interaction surface](SPEC.md#configure-and-reuse-roles) is settled:
Conduit's CLI launches native agent interfaces. Resolve account separation and sandbox
guarantees first, then launch configuration and the minimal memory backend. Choose memory
read/write/search operations and their returned evidence before selecting a graph, vector engine,
ontology, or broad plugin schema. Pi and own chat follow the initial workflow.

The immediate ruling is the [execution-isolation contract](DESIGN.md#account-separation-and-execution-isolation):
whether v0 offers native/profile-separated and OS-isolated modes, which is the default,
and what roles may require. Account separation is mandatory in either mode. Decide
local files/processes/services first, then network egress and provider-operation limits.
Choose a backend only after defining guarantees and required native capabilities.
Also resolve private state when one account serves differently scoped sessions, auth
refresh ownership, and scope changes that need an execution restart.

### 3. Bind collaboration to resources

Resolve these connected questions for that workflow:

| Boundary | What remains to decide |
| --- | --- |
| Role and connection setup | Configuration precedence, credential references/concurrent logins, skill import/update behavior, and capability checks when switching runtimes. |
| Managed access | Which operations/selectors/sharing modes v0 supplies; what chosen integrations can actually restrict; initial observation coverage and MCP routing. |
| Room and work | Membership and visibility, addressing, relation to task/workspace/issue, and the transition to mandatory tracker linkage. Start without a tracker for room testing; Linear remains the first integration. |
| Memory retrieval | Search/read/save contract, relevance and context budgets, source freshness, and handling partial results. Available content must remain distinct from automatically loaded context. |

Shared connection selection is settled in the [integration contract](DESIGN.md#ownership-and-extension-boundaries):
scopes select reusable connections and grants, roles set limits, and runtime integrations
prepare native setup. Project defaults and launch-override precedence remain open.

YAML and [versioned, named objects](DESIGN.md#configuration) are settled. Choose the
Conduit schema-version convention, fields, file layout, and precedence/merge rules.

Reusable connection definitions do not require shared live MCP clients. Review the
[native registration findings](DESIGN.md#mcp-and-native-compatibility), then choose
per-launch configuration and authentication handling for concurrent Claude/Codex roles.
Native configuration must account for inherited and plugin-provided servers; a launch
override alone does not establish an exclusive server list or agent-wide enforcement.

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

Remote rooms, role-specific wake hooks, comprehensive MCP operation enforcement,
advanced sharing delivery, and automatic classification/extraction stay staged.
Their principles remain in the specification/design. V0 sandbox guarantees and backend
validation are now on the critical path; full event schemas, exact costs, and plugin
ABI details remain later decisions.

## Editing the specification

Resolve one coherent boundary at a time. Edit the relevant contract in place and
remove answered questions here. Ask before changing an ambiguous substantive
requirement. Use Git history for previous versions; do not recreate a numbered
chronology or duplicate the specification in a separate decision register.
