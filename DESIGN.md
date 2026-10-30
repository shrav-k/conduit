# Conduit architecture and contracts

Updated: 2026-10-30. Read [SPEC.md](SPEC.md) for vocabulary and expected behavior.
This describes current boundaries; [REVIEW.md](REVIEW.md) contains unresolved choices.
Intersession communication contracts are deferred there for redesign after Pi research.

## Ownership and extension boundaries

The native Rust core coordinates identity, sessions, shared records,
and managed resource access. Integrations preserve provider-native behavior. These
are responsibilities, not a required collection of processes or crates.

| Owner | Responsibility |
| --- | --- |
| Conduit | Local roles/connections, logical sessions, scopes, shared knowledge/checkpoints, and available observations. Communication responsibilities will be refined in the deferred review. |
| Native agent | Planning, execution loop, private working context, native skills/tools, and native subagents. |
| Resource or tracker provider | Source content and native operations; Linear issue fields, assignments, status, and workflow. |
| Storage/search plugin | Persistence and retrieval under Conduit's supported record/access contracts, with explicit capability and freshness limits. |
| Frontend/runtime integration | Presentation, launch/attachment, account selection, input/control, and supported lifecycle/events. |

Plugins may integrate models, runtimes, frontends, resources, trackers, and storage/search.
A connection definition describes reusable local setup for a service/backend; it does
not represent an already-open MCP client. Scopes select
connection definitions and grant their permitted tools/resources; roles
supply execution settings, native skills, and optional permission ceilings. A role
does not need a second list of shared connections. Definition alone grants no access.

Resolve the selected scopes against role limits for each session, then let its runtime
integration prepare native MCP setup. Keep host-specific transport, configuration, or
authentication handling in that integration; report unsupported capabilities. Changing
the model alone need not change registration. Each native host manages its MCP clients;
shared definitions do not merge permissions or promise shared authentication/client state.

Xerj remains a candidate behind a replaceable storage/search plugin. No database,
plugin ABI, transport, process model, or hosting arrangement is selected. Authoritative
records, observations of external state, and search projections have different meanings.

### Configuration

YAML is the authoring format for versioned, named configuration objects with explicit
references and schema validation. Keep authored configuration separate from observed
runtime state. [Kubernetes object structure](https://kubernetes.io/docs/concepts/overview/working-with-objects/)
is an inspiration; its tooling and role model are not dependencies. The version
identifier scheme, exact fields, file layout, precedence, and merge rules remain open.
Conduit configuration versions are separate from MCP protocol and plugin versions.

For a project launch, use its configured default scopes only when no explicit scope
selection is supplied. An explicit selection replaces those defaults; extending the
defaults requires an explicit add action. Apply role limits and exclusions to the
result in every case. This keeps convenient project setup from silently retaining
access the caller intended to replace. Exact authoring and CLI syntax remain open.

A role explicitly selects the user-level native settings, skills, and plugins it
reuses; do not automatically inherit the user's default agent profile. An existing
setup may be selected once for convenience, preserving its native formats. This makes
role composition explicit without requiring a replacement skill/plugin framework.
Shared MCP connections, including definitions bundled with selected plugins, still
participate in scope selection; selecting a source grants no additional access.
Authentication and private history remain separately managed, and mandatory host or
organization policies still apply. Adapters must validate native loading/precedence.

V0 uses each ecosystem's native tooling to install and update selected skills/plugins.
Conduit adapters may invoke that tooling on request and record available source/version
information and the outcome. Conduit does not introduce its own package manager for
these native sources. This preserves existing installation conventions while allowing
users to manage their selected sources through Conduit.

## Account separation and execution isolation

Concurrent account separation is required in v0. Keep a role's reusable configuration,
an account's persistent login, and an execution's effective scopes/state distinct.
Never implement switching by swapping credentials in one global agent directory.
Bind Conduit requests to authenticated session/execution identity rather than trusting
a caller-supplied role or session label. Model-provider credentials do not select
resource-provider credentials.

For v0 Claude/Codex subscriptions, native runtimes own sign-in and token refresh.
Conduit manages named account references, selects the intended profile, and verifies
the selected identity through supported runtime evidence. Keep provider authentication
behind runtime adapters instead of implementing a separate subscription OAuth system.
This preserves native account behavior while giving Conduit responsibility for correct
selection and attribution. Concurrent accounts and credential reuse with session-private
state still require adapter validation; successful native login alone proves neither.

Stored native conversation history and agent-maintained private memory belong to the
Conduit session, even when sessions use the same role or subscription. Resume may reuse
that session's compatible state; fresh sessions do not inherit it automatically, and
forks use the already-defined checkpoint semantics. Reusable knowledge is shared through
scoped Conduit memory. Credential reuse is a separate concern and does not authorize
sharing private state. This keeps role/account reuse from crossing different session
scopes. Adapters must validate the storage/loading separation; native mode does not
thereby acquire an OS boundary against otherwise accessible host files.

[Claude documents separate subscription profiles](https://code.claude.com/docs/en/authentication#log-in-with-multiple-accounts)
through `CLAUDE_CONFIG_DIR`. [Codex state locations](https://learn.chatgpt.com/docs/config-file/environment-variables)
use `CODEX_HOME`; its [credential backend](https://learn.chatgpt.com/docs/auth#credential-storage)
can be a file, OS store, or process memory. These are adapter primitives, not proof
of OS isolation. Validate credential namespaces, refresh behavior, inherited environment,
native memory, plugins, and daemon attachment for each supported runtime/version.

V0 offers both modes below, with **native execution as the default**. Make the role's
configured mode and each execution's effective mode visible and record the latter.
Isolated execution is an explicit option for additional protection. Both modes retain
the account-separation and Conduit-access guarantees above; neither merges scopes.

A role may require isolated execution. Launch overrides cannot select native mode for
that role; unavailable required isolation causes a clear launch failure. Without that
requirement, either mode may be selected, with native remaining the general default.
The requirement is separate from a preferred mode: defaults support convenient setup,
while an explicit requirement preserves the role's boundary across launch interfaces.

| Mode | Boundary and tradeoff |
| --- | --- |
| Native with separate profiles | Keeps each account's setup and state separate while retaining host integration. Native tools still have their actual host privileges; this is not an OS security boundary. |
| Isolated execution | Encloses the runtime and its child processes with explicit access to selected files/services and declared network destinations, excluding other accounts' state and privileged host controls. Some host integrations need bridges or remain unavailable. The isolation backend remains unselected. |

Isolated execution defaults to declared network destinations: required model/login
services, endpoints for selected connections, and extra destinations granted through
scopes, subject to role limits and exclusions. Make runtime endpoint requirements
visible; selecting a runtime must not silently override a restriction. Package
registries, research sites, and local/private services require explicit inclusion.
An allowed destination does not authorize every operation or object at that service.

This default gives deliberately isolated executions a defined network boundary while
native mode preserves broad host compatibility. Exact endpoint matching and enforcement
mechanisms belong to backend/configuration design; provider-operation inspection remains
a separate capability.

### Why both modes, with native as the default

The first workflow needs multiple subscriptions to coexist reliably. Separate profiles
solve identity/configuration collisions without requiring every user to isolate their
entire development environment. Native execution preserves host integrations and suits
users who deliberately grant broad local access. Making full isolation mandatory would
add setup and compatibility costs to that common case, so v0 defaults to native.

Profile separation does not stop native tools from reaching other accessible host files.
Isolated execution supplies that stronger boundary for users who need it. It covers
the runtime and children because a command-only sandbox leaves other tool paths outside
the boundary. IDE sockets, browser authentication, keychains, Docker, local services,
and native OS tooling may need explicit access or bridges. Supporting native mode keeps
these compatibility gaps from excluding an otherwise useful agent integration.

Keep local isolation, network destinations, provider-operation permissions, and Conduit
record access distinct: each controls a different path. Visibility of the selected mode
and actual coverage prevents profile separation from being mistaken for OS security.
Account, role, and execution also remain distinct so reusing a subscription does not
automatically load another role's private context. These boundaries preserve native
workflows while allowing stronger, replaceable isolation backends.

An isolated mode cannot rely only on native command sandbox flags:
[Claude's Bash sandbox](https://code.claude.com/docs/en/sandboxing#what-runs-outside-the-sandbox)
excludes file tools, MCP servers, and hooks; [Codex command permissions](https://learn.chatgpt.com/docs/permissions)
also leave other surfaces under separate controls. Whole-process OS wrappers,
containers, or VMs are candidates, not selected dependencies.

Resolve the chosen mode and required capabilities before starting work; reject
unsupported required isolation rather than silently downgrade.
Use the session's private native state and only intended credentials/resources. Do not
mount an account's entire native history into every role using it. Coordinate native
credential access instead of blindly cloning rotating credentials. The exact
credential/state layout needs adapter validation. For YAML authoring, explicitly
writing the mode in generated roles is recommended; whether handwritten configurations
may omit it remains a schema choice.
Filesystem isolation does not itself restrict operations at an allowed remote service.

## Sessions and workspaces

One logical session can span multiple native executions. It has at most one active
primary agent; independently coordinated workers have separate sessions. Native
subagents may remain inside their parent's execution.

One frontend controls direct input, cancellation, and configuration changes; other
frontends observe. Continuing in another frontend transfers control without a separate
confirmation. Disconnecting detaches while preserving the session and supported ongoing
execution; unknown/interrupted state must be reported without launching a duplicate.

Session lifecycle is **open**, **paused**, or **closed**, separate from execution state
and frontend attachment. Paused/closed sessions need explicit resume/reopen. Closing
a session does not complete its task or delete records. Resume/fresh/fork behavior is
defined in the [specification](SPEC.md#share-knowledge-and-continue-work).

A task or experiment owns its workspace/worktree; sessions attach to it. Changing
agent, account, or frontend need not create a checkout. Forking a conversation does
not fork files, and a worktree does not imply isolation. Concurrent editing rules,
workspace attachment, and cleanup remain open.

Linear is the first tracker direction. Providers own issue fields, assignments,
status, and workflow; Conduit does not require an internal ticket engine or full
replica. Tracker relationships to communication are deferred to REVIEW.md.

## Memory and record lifecycle

Conduit retains knowledge, checkpoints, usage observations, and artifact
references. Plugins may add namespaced record kinds. An internal work-item engine is
optional. Type-specific payloads share small metadata for identity, ownership, revision,
provenance, and links; external provider objects need not adopt this schema.

- **Identity:** stable record IDs survive renames and moves. Each record has one owning
  context, meaning a task, project, or personal collection used as its ownership anchor.
  This is distinct from an agent's working context. References can be live or pinned
  to a revision; unavailable pinned content is never silently replaced.
- **Updates:** mutable kinds create immutable revisions. A stale base revision returns
  a conflict; agents may reconcile and retry without mandatory human approval.
- **Evidence:** distinguish original source/author, revision writer, and derivation
  inputs. Local recording time differs from labeled source times. Unknown provenance
  stays unknown and does not certify truth or approval.
- **Cleanup:** archive excludes ordinary retrieval while preserving explicit scoped
  access. Explicit purge removes retained content/history with a minimal deletion
  marker. Removing an artifact reference does not delete its referenced file. Artifact
  records are independently searchable/linkable; a reference does not promise a snapshot.

A checkpoint contains optional objective/constraints, decisions, progress/evidence,
remaining work/questions, and artifact/source references. The active agent writes it
at planned handoff, explicit pause, task completion, or on demand; Conduit requests
and saves it with known provenance. Failed capture continues with available permitted
context and clearly marked gaps rather than requiring a separate summarizer.

Knowledge is saved explicitly by users/agents. Classification is optional and entirely
user-defined, supports multiple dimensions and labels per dimension, and requires no
advance declaration. Optional definitions are scoped knowledge records linked explicitly;
matching names do not bind automatically. Background extraction/classifier interfaces
are deferred.

Memory search defaults to all Conduit memory permitted by the session's selected scopes,
with optional narrowing by context, record kind, or classification labels. This lets
agents discover relevant knowledge without knowing its location first. Narrowing never
expands access; a memory query does not automatically search every connected repository
or external service.

Results contain bounded excerpts with record/revision references and permitted
provenance; full content requires an explicit read. This lets agents evaluate relevance
before spending context on a whole record. Both search and subsequent reads apply
current access rules, including restrictions on provenance. Exact budgets, freshness
reporting, and partial-result behavior remain open; this contract does not select a
storage or search backend.

## Access and sharing

Effective access is the union of selected scope grants, constrained by declared role
limits and explicit exclusions. Missing role limits add no grants. Permissions name
operations per resource type; convenience bundles expand into visible grants. Permitted
operations do not require an extra human confirmation merely because permissions exist.

Selectors support IDs, named ownership contexts, simple filters, and path patterns
where integrations support them. Resource matching and authorized scope/role-limit
changes apply live to subsequent managed requests, with effective changes recorded
and shown. They do not change model/account defaults or automatically inject context.

If a scope change removes access that an active isolated execution cannot revoke live,
stop the affected execution and report that an explicit restart is required. Preserve
the logical session; do not automatically launch a replacement.
Restart resolves the current rules. This prevents continued execution with obsolete
access while making the interruption visible. Conduit-managed requests use the new
rules immediately, independently of the restart.

Selecting a context covers directly owned records by default. Descendants and incoming
shares need explicit inclusion, which presets can provide. Ordinary links grant no
access. Unsupported/restricted bindings remain unavailable while other usable access
proceeds; integrations must not silently drop restrictions.

Check whole-record access using the session's current permissions when retrieving or
loading memory/context, including checkpoint reuse. Mixed-access content is withheld
as a whole; narrower context requires a separately authorized extract. Apply the same
access rules to search, caches, transcripts, and
derived records. Permission changes cannot erase already-delivered native context.

### Sharing semantics

These define supported sharing behavior as it is introduced; the full set is not a
v0 requirement.

| Operation | Contract |
| --- | --- |
| Live or pinned share | Default to read access to the original with its ID/owner intact; live follows updates, pinned selects a source revision. Additional operations are explicit, subject to recipient limits. |
| Share management | Separate authority from read/update. A manager may grant only operations it currently holds. Creating, changing, and withdrawing shares are managed operations. |
| Onward delegation | Explicit permission permits grants within received rights, linked to the parent. A recipient manages its onward shares, not the incoming grant. Parent restrictions propagate; other valid grants remain independent. |
| Grant lifecycle | Issued grants survive changes to their creator's session/role. Parent-constrained access recovers when the same chain permits it; explicit withdrawal requires authorized restoration/reissue. New parent grants do not silently replace old links. |
| Derived knowledge or copy | Require read access to the record and all declared source inputs. A copy has its own ID/owner and editable content but retains access dependencies. |
| Independent publication | Create a separate extract/audience with retained provenance. Require read/publication authority for each released restricted input plus destination creation rights. It does not follow source updates and retains its own access/lifecycle after source restriction or deletion. |
| Published updates | Normal edits use update permission; newly introduced restricted inputs require current read/publication authority and destination update rights. Prior publication does not release other source content. |
| Missing evidence or authority | A missing source payload does not block stored derived content if current access remains verifiable. Unverifiable required permissions withhold content without deleting it. Historical provenance gaps alone do not block otherwise permitted use. |
| Provenance display | Restrict source identifiers, titles, paths, URLs, and previews to authorized readers; show a generic notice for hidden lineage. Explicitly published attribution remains readable. |

## MCP and native compatibility

MCP is the default external capability contract and preferred plugin boundary. Use
standard tools, resources, prompts, and supported extensions before inventing a generic
Conduit protocol. Native adapters address concrete lifecycle/account/hook or non-MCP
needs and should expose MCP where practical. Internal Rust modules need not be servers.

Registration configures a native host to launch a server command or contact an endpoint;
authentication is a separate concern. [MCP's architecture](https://modelcontextprotocol.io/docs/2026-07-28/learn/architecture)
supports multiple clients of an HTTP service, while stdio usually connects a client to
its launched subprocess. Reusing a definition or endpoint does not pool live clients.
Multiple server processes need a shared backend to expose common durable state.

[Codex](https://learn.chatgpt.com/docs/extend/mcp?surface=cli) stores registrations in
user/project TOML; [Claude Code](https://code.claude.com/docs/en/mcp) supports persistent
registrations and launch-specific `--mcp-config` with `--strict-mcp-config`.
[Codex configuration overrides](https://learn.chatgpt.com/docs/config-file/config-basic#configuration-precedence)
can supply launch values, but are not by themselves an isolated configuration source.
Generate native launch configuration from effective scopes without
repeatedly rewriting global settings. Validate inherited/plugin servers, concurrent
accounts, credential reuse, and host lifecycle before selecting the binding mechanism.
Do not assume a separate live client per conversation or cross-host OAuth portability.

Preserve native provider tool names, descriptions, schemas, results, and workflows
wherever possible; identify necessary adaptations. The target is interception of all
outgoing MCP calls from managed role sessions, with attribution and eventual permission
checks that allow, limit, or block calls. This does not require one combined endpoint:
a common gateway or multiple provider endpoints can provide mediation.

V0 need not implement comprehensive inspection or enforcement of individual upstream
operations. Its account separation and execution-isolation choices are addressed
[above](#account-separation-and-execution-isolation). Access to Conduit-managed resources
remains core. Each mode must state observed activity, enforced restrictions, and
uncovered paths. Preventing bypass requires more than configuring MCP connections;
isolating local processes does not replace provider authorization or MCP mediation.

Conduit's sessions, ownership, and scope semantics are application contracts.
Bind requests to effective session identity/access; do not equate an MCP connection
with a Conduit session. Protocol authorization does not supply Conduit's complete
access policy.

Candidate mappings are tools for memory search/writes, resources for readable records,
and prompts for reusable instructions. Exact schemas and the protocol/extension baseline
must be verified with the selected hosts. Native BYOS remains usable regardless of
optional MCP skill-discovery support. No host/SDK compatibility has been tested.

## Observation and recovery

Record and attribute available token usage across agents, preserve available transcripts,
and associate observations with session/execution identity. Initial coverage depends
on integrations; comprehensive tools/subagent tracking, MCP inspection, and user hooks
remain broader goals. Payload retention and exact cost accounting remain open.

The proposed v0 evidence distinguishes measured usage, estimates, unknown coverage,
and observed zero; native child attribution and deduplication need adapter validation.
Likewise distinguish confirmed effects, failures, and unknown outcomes. A timeout
does not establish that a provider action failed. Retry identity, duplicate handling,
restart behavior, and retention must be chosen for the first workflow.

## Design references

References inform the design; they do not select dependencies or prove compatibility.

- [Spec Kit](https://github.com/github/spec-kit): structure around scenarios, requirements,
  and a separate technical plan. No toolkit installation or generated workflow is required.
- [MCP architecture](https://modelcontextprotocol.io/docs/2026-07-28/learn/architecture)
  and [tools](https://modelcontextprotocol.io/specification/2026-07-28/server/tools):
  standard capability exchange and provider tool contracts; actual host support needs verification.
- [Xerj](https://github.com/xerj-org/xerj): a replaceable memory/search backend candidate.
- [Superpowers](https://github.com/obra/superpowers),
  [Matt Pocock's skills](https://github.com/mattpocock/skills), and
  [Ponytail](https://github.com/DietrichGebert/ponytail): native skills and integration references.
- Local `jxp-skills` and `pstack-pi` checkouts inform usage accounting and native
  integration. Their storage, deployment, and skill-package designs are not Conduit
  requirements. Communication references, including `agentwatch`, are in REVIEW.md.
