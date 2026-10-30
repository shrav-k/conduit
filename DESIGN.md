# Conduit architecture and contracts

Updated: 2026-10-30. Read [SPEC.md](SPEC.md) for vocabulary and expected behavior.
This describes current boundaries; [REVIEW.md](REVIEW.md) contains unresolved choices.
Conduits are the accepted common relationship model for resource access and session
communication. Their detailed contracts are the next research priority in REVIEW.md,
building on the [Pi investigation](RESEARCH_PI.md).

## Conduits as the core interaction model

A conduit is a typed, authorized relationship for accessing resources or exchanging
information between sessions. This is the central organizing concept of the project,
replacing the provisional access “attachment” term and extending beyond the former
room model. Access and communication share a conceptual foundation; they need not
share every operation, persistence rule or delivery guarantee.

Pools collect domain resources; roles configure sessions; conduits establish permitted
interactions; filters only narrow them. Connection definitions describe the service
setup a conduit may use, while plugins implement supported protocols and resource
contracts. A conduit is not synonymous with an MCP client, socket, frontend attachment,
or active model turn. Common concepts do not require a single gateway or transport.

Research must establish endpoint identity, addressing, interaction contracts, lifecycle,
authority, observation and failure semantics before fixing conduit kinds or YAML fields.
Distinguish authored declarations from runtime instances and observed status. The
requirements for account separation, current managed-access checks, explicit session
lifecycle and honest usage remain applicable. The common concept does not itself
restore earlier room rules or authorize automatic wakeups.

## Ownership and extension boundaries

The Rust core owns Conduit's runtime as well as identity, sessions, shared records,
and managed resource access. Frontends express that runtime through a control/state
boundary. Integrations preserve provider-native behavior. These are responsibilities,
not a required collection of processes or crates.

| Owner | Responsibility |
| --- | --- |
| Conduit runtime | Session supervision, execution loop, context construction, provider/tool dispatch, lifecycle, recovery and available observations, implemented in Rust. Models supply reasoning within that runtime. |
| Conduit shared services | Conduit relationships, roles/connections, logical identities, filters, knowledge/checkpoints and managed resource access. Interaction-specific communication contracts follow the research review. |
| Provider/account adapter | Model request/response conversion, capabilities, supported authentication, account selection and usage evidence. |
| External-agent adapter | Connect a separate runtime such as Claude Code/Codex, which retains its own loop and private native state. Report the lifecycle/access/telemetry capabilities actually supported. |
| Resource or tracker provider | Source content and native operations; Linear issue fields, assignments, status, and workflow. |
| Storage/search plugin | Persistence and retrieval under Conduit's supported record/access contracts, with explicit capability and freshness limits. |
| Frontend | Presentation, input and supported interaction; request launch/attachment/settings changes through Conduit. It does not own credentials, tools, model calls, or authoritative runtime state. |

Plugins may integrate models, runtimes, frontends, resources, trackers, and storage/search.
A connection definition describes reusable local setup for a service/backend; it does
not represent an already-open MCP client. Authorized conduits for resource access select connection
definitions and grant resource operations; filters only narrow that access. Roles
supply execution settings, native skills, and optional permission ceilings. A role
does not need a second list of shared connections. Definition alone grants no access.

Resolve selected conduits and filters against role limits for each session.
Conduit's runtime owns its MCP clients; external-agent adapters prepare native registration where needed.
Keep host-specific transport/configuration/authentication in the relevant adapter and
report unsupported capabilities. Changing the model alone need not change registration.
Shared definitions do not merge permissions or promise shared authentication/client state;
a frontend does not independently connect selected upstream services.

Xerj remains a candidate behind a replaceable storage/search plugin. No database,
plugin ABI, transport, process model, or hosting arrangement is selected. Authoritative
records, observations of external state, and search projections have different meanings.

The first frontend is a custom Conduit terminal inspired by Pi. Pi's agent loop and
Pi Durable are not runtime dependencies. The stock terminal directly uses Pi runtime
services, so selective reuse of `pi-tui` and presentation components is a candidate
for our frontend. Full stock-UI preservation is unnecessary; exact dependencies remain
open. Pi Durable, Chord and pi-env inform our own implementation; they do not determine Conduit's state model or public contracts.

### Runtime and presentation

Conduit-owned execution resolves configuration, admits input, assembles authorized
context, calls the selected provider, dispatches tools, records outcomes and continues
until settled or waiting for input. Private execution history, model-request context,
frontend state and shared knowledge have separate responsibilities. Replacing a memory
backend or frontend must not replace the execution loop.

Existing lifecycle, safe-retry, access and usage contracts apply to our runtime as well
as managed external integrations. The research proposes persisted tool intents/results,
explicit child ownership, bounded frontend snapshots/updates and context source manifests.
Exact internal states, recovery granularity, frontend transport and storage engine remain
review/implementation choices. We need the smallest runtime that satisfies the first
workflow, not every Pi/Durable feature.

BYOS remains an input compatibility goal. Plain instructions may be reusable; executable
Pi/Claude/Codex extensions that call native runtime APIs require an adapter or port.
Frontend reuse does not supply that compatibility. Preserve source/version information
and report unsupported hooks rather than loading an implicit second agent runtime.

### Pools and resource extensibility

A pool is a named, extensible collection of resources relevant to interacting with a
domain. It replaces the former Conduit project grouping. Resource contracts describe
what each member exposes; the pool organizes references without requiring a common
backend or a fixed catalog of content categories. New resource kinds should fit the
same membership mechanism rather than require fields added to every pool schema.

Data, tools, skills and documents illustrate possible members, not an exhaustive type
system. A use such as onboarding is ordinary resource content/metadata rather than a
special core kind or pool field. Pool-provided skills extend BYOS availability beyond
role-only bundles; compatible activation and composition rules remain for review.

Membership does not by itself copy content, change its owner, grant operations, or
load information into a model. Existing record ownership and access contracts still
apply. Pool membership supports both explicit resource references and label selectors.
Exact selector syntax, update behavior, overlapping membership and any nesting semantics
remain to be specified. Membership selection is distinct from session access filtering.

### Configuration

YAML is the authoring format for versioned, named configuration objects with explicit
references and schema validation. Keep authored configuration separate from observed
runtime state. [Kubernetes object structure](https://kubernetes.io/docs/concepts/overview/working-with-objects/)
and [API versioning](https://kubernetes.io/docs/reference/using-api/#api-versioning)
are explicit inspirations, alongside networking APIs such as
[Gateway API](https://gateway-api.sigs.k8s.io/docs/concepts/api-overview/).
Study generic identity/metadata, typed references, selectors, desired versus observed
state, extensible resource kinds, API groups and schema evolution. Apply those patterns
to Conduit's contracts; Kubernetes machinery is not required. The version identifier
scheme, exact fields, file layout, precedence and merge rules remain open. Keep schema
versions distinct from application releases, record revisions, MCP and plugin versions.

For a pool launch, use its configured access defaults only when no explicit access
selection is supplied. An explicit selection replaces those defaults; extending the
defaults requires an explicit add action. Apply role limits and exclusions to the
result in every case. This keeps convenient pool setup from silently retaining
access the caller intended to replace. Exact authoring and CLI syntax remain open.

A role explicitly selects the user-level native settings, skills, and plugins it
reuses; do not automatically inherit the user's default agent profile. An existing
setup may be selected once for convenience, preserving its native formats. This makes
role composition explicit without requiring a replacement skill/plugin framework.
Shared MCP connections, including definitions bundled with selected plugins, still
participate in access selection; selecting a source grants no additional access.
Authentication and private history remain separately managed, and mandatory host or
organization policies still apply. Adapters must validate native loading/precedence.

V0 uses existing ecosystem tooling to install and update compatible selected skills/plugins.
Conduit adapters may invoke that tooling on request and record available source/version
information and the outcome. Conduit does not introduce its own package manager for
these native sources. This preserves existing installation conventions while allowing
users to manage their selected sources through Conduit.

## Account separation and execution isolation

Concurrent account separation is required in v0. Keep a role's reusable configuration,
an account's persistent login, and an execution's effective access/state distinct.
Never implement switching by swapping credentials in one global agent directory.
Bind Conduit requests to authenticated session/execution identity rather than trusting
a caller-supplied role or session label. Model-provider credentials do not select
resource-provider credentials.

For external Claude Code/Codex integrations, native runtimes continue to own sign-in
and token refresh. Conduit-owned execution needs a supported provider/account adapter
instead; the previous native-auth delegation does not automatically cover that path.
Keep authentication out of frontends and shareable YAML. Validate subscription eligibility,
identity evidence, credential namespaces and refresh ownership before the live experiment;
do not assume Pi's implementation can simply be copied or native tokens reused.

Conduit manages named account references and verifies the identity actually selected.
Concurrent accounts and credential reuse with session-private state still require
validation. A successful login alone proves neither distinct-account selection nor
independent refresh. No silent API-billing or account fallback is permitted.

Stored conversation history and agent-maintained private memory belong to the
Conduit session, even when sessions use the same role or subscription. Resume may reuse
that session's compatible state; fresh sessions do not inherit it automatically, and
forks use the already-defined checkpoint semantics. Reusable knowledge is shared through
scoped Conduit memory. Credential reuse is a separate concern and does not authorize
sharing private state. This keeps role/account reuse from crossing session access boundaries. Adapters must validate the storage/loading separation; native mode does not
thereby acquire an OS boundary against otherwise accessible host files.

For external native-agent adapters, [Claude documents separate subscription profiles](https://code.claude.com/docs/en/authentication#log-in-with-multiple-accounts)
through `CLAUDE_CONFIG_DIR`. [Codex state locations](https://learn.chatgpt.com/docs/config-file/environment-variables)
use `CODEX_HOME`; its [credential backend](https://learn.chatgpt.com/docs/auth#credential-storage)
can be a file, OS store, or process memory. These are adapter primitives, not proof
of OS isolation. Validate credential namespaces, refresh behavior, inherited environment,
native memory, plugins, and daemon attachment for each supported runtime/version.

V0 offers both modes below, with **native execution as the default**. Here native
means execution on the host without Conduit's full OS sandbox, not selection of a
particular agent runtime. Both Conduit-owned and external-agent execution need explicit
coverage. Make the role's configured mode and each execution's effective mode visible
and record the latter.
Isolated execution is an explicit option for additional protection. Both modes retain
the account-separation and Conduit-access guarantees above; neither merges permissions.

A role may require isolated execution. Launch overrides cannot select native mode for
that role; unavailable required isolation causes a clear launch failure. Without that
requirement, either mode may be selected, with native remaining the general default.
The requirement is separate from a preferred mode: defaults support convenient setup,
while an explicit requirement preserves the role's boundary across launch interfaces.

| Mode | Boundary and tradeoff |
| --- | --- |
| Native with separate account/session state | Keeps each account's setup and state separate while retaining host integration. Tools and plugins still have their actual host privileges; this is not an OS security boundary. |
| Isolated execution | Encloses the runtime and its child processes with explicit access to selected files/services and declared network destinations, excluding other accounts' state and privileged host controls. Some host integrations need bridges or remain unavailable. The isolation backend remains unselected. |

Isolated execution defaults to declared network destinations: required model/login
services, endpoints for selected connections, and extra destinations granted through
conduits, constrained by filters, role limits and exclusions. Make runtime endpoint
requirements visible; selecting a runtime must not silently override a restriction. Package
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
Use the session's private execution state and only intended credentials/resources. Do not
mount an account's entire native history into every role using it. Coordinate native
credential access instead of blindly cloning rotating credentials. The exact
credential/state layout needs adapter validation. For YAML authoring, explicitly
writing the mode in generated roles is recommended; whether handwritten configurations
may omit it remains a schema choice.
Filesystem isolation does not itself restrict operations at an allowed remote service.

## Sessions and workspaces

One logical session can span multiple executions, with their runtime/model/account
bindings recorded separately. It has at most one active primary agent; independently
coordinated workers have separate sessions. Runtime-owned subagents may remain inside
their parent's execution, with attributable ownership and usage where available.

One frontend controls direct input, cancellation, and configuration changes; other
frontends observe. Continuing in another frontend transfers control without a separate
confirmation. Disconnecting detaches while preserving the session and supported ongoing
execution; unknown/interrupted state must be reported without launching a duplicate.

Session lifecycle is **open**, **paused**, or **closed**, separate from execution state
and frontend attachment. Paused/closed sessions need explicit resume/reopen. Closing
a session does not complete its task or delete records. Resume/fresh/fork behavior is
defined in the [specification](SPEC.md#share-knowledge-and-continue-work).

A task or experiment owns its workspaces/worktrees; sessions attach to them. Changing
agent, account, or frontend need not create a checkout. Forking a conversation does
not fork files, and a worktree does not imply isolation.

V0 can attach an existing checkout/folder or create a Conduit-managed Git worktree,
recording the management status. Attachment preserves existing files and Git state
without granting deletion authority. Supporting both preserves existing workflows
while providing managed creation for new work. Exact creation settings belong to
the YAML review.

For concurrent coding sessions on Git projects, independent writers use separate
task-owned worktrees by default; sequential handoffs reuse a workspace. This separates
concurrent edits while keeping workspace ownership independent of roles and accounts.
Shared-workspace editing is available through explicit selection, without a guarantee
of automatic merging or file locking.

Workspaces remain after session closure or task completion until an explicit cleanup
operation. Cleanup of an attached directory only unregisters it from Conduit, leaving
its files intact. Before removing a managed worktree, check that no execution is using
it and no unique local work would be lost. If either cannot be established, preserve
the worktree and report why. This keeps cleanup separate from conversation/task lifecycle
and preserves work when its recovery status is uncertain.

Linear is the first tracker direction. Providers own issue fields, assignments,
status, and workflow; Conduit does not require an internal ticket engine or full
replica. Tracker relationships to communication are deferred to REVIEW.md.

## Memory and record lifecycle

Conduit retains knowledge, checkpoints, usage observations, and artifact
references. Plugins may add namespaced record kinds. An internal work-item engine is
optional. Type-specific payloads share small metadata for identity, ownership, revision,
provenance, and links; external provider objects need not adopt this schema.

- **Identity:** stable record IDs survive renames and moves. Each record has one owning
  context, meaning a task, pool, or personal collection used as its ownership anchor.
  This is distinct from an agent's working context. References can be live or pinned
  to a revision; unavailable pinned content is never silently replaced.
- **Updates:** mutable kinds create immutable revisions. A stale base revision returns
  a conflict; agents may reconcile and retry without mandatory human approval.
- **Evidence:** distinguish original source/author, revision writer, and derivation
  inputs. Local recording time differs from labeled source times. Unknown provenance
  stays unknown and does not certify truth or approval.
- **Cleanup:** archive excludes ordinary retrieval while preserving explicit scoped
  access; restore returns it to ordinary retrieval under current access rules. Explicit
  purge removes retained content/history with a minimal deletion marker. Removing an
  artifact reference does not delete its referenced file. Artifact records are
  independently searchable/linkable; a reference does not promise a snapshot.

V0 knowledge records support create, read/search, revision-checked update, archive,
restore, and explicit purge, each governed by its operation permissions. This gives
agents a way to correct and retire knowledge as well as save it. Bulk administration
and advanced sharing remain separate; other record kinds retain their type-specific
operations and update rules.

V0 retains records, their revisions, and captured history under Conduit's retention
control until explicit purge. This includes knowledge, checkpoints, captured transcripts,
usage observations, and operation outcomes. Archive changes ordinary retrieval, not
retention. Automatic expiry is deferred so the initial experiment preserves continuity
and evidence. Integrations must report native-runtime or backend retention limits outside
Conduit's control; this policy does not promise preservation of unavailable source history.

A checkpoint contains optional objective/constraints, decisions, progress/evidence,
remaining work/questions, and artifact/source references. The active agent writes it
at planned handoff, explicit pause, task completion, or on demand; Conduit requests
and saves it with known provenance. Failed capture continues with available permitted
context and clearly marked gaps rather than requiring a separate summarizer. This
portable task checkpoint is distinct from internal execution recovery state and from
a model-context compaction summary; neither automatically satisfies its content contract.

Knowledge is saved explicitly by users/agents. Classification is optional and entirely
user-defined, supports multiple dimensions and labels per dimension, and requires no
advance declaration. Optional definitions are scoped knowledge records linked explicitly;
matching names do not bind automatically. Background extraction/classifier interfaces
are deferred.

Memory search defaults to all Conduit memory permitted by the session's effective access,
with optional narrowing by context, record kind, or classification labels. This lets
agents discover relevant knowledge without knowing its location first. Narrowing never
expands access; a memory query does not automatically search every connected repository
or external service.

Results contain bounded excerpts with record/revision references and permitted
provenance; full content requires an explicit read. This lets agents evaluate relevance
before spending context on a whole record. Common configurable defaults and caps bound
result count and total returned text across backends. Agents may request different
limits within those caps; responses identify when a limit was reached. This keeps
context use predictable without fixing one response size for every query. Both search
and subsequent reads apply current access rules, including restrictions on provenance.

When some selected memory cannot be searched, return available results with an explicit
incomplete-search status. Report known coverage gaps and backend-supplied freshness;
unknown coverage or freshness remains unknown. Diagnostic details respect the reader's
permissions. An unavailable backend is never reported as “no matches,” so agents do not
mistake missing evidence for an empty search. Exact limit values and text-size units
belong to the YAML review and experiment. These contracts do not select a storage or
search backend.

## Access and sharing

Conduits supply authorized resource/connection access and operation grants according
to their interaction contracts.
Filters are narrowing-only: they may remove resources or operations from a target
selection, but cannot introduce grants. A filter alone supplies no access. Declared
role limits and explicit exclusions constrain effective access; missing role limits
add no grants. Permissions name operations per resource type, with convenience bundles
expanding into visible grants. Permitted operations do not require an extra human
confirmation merely because permissions exist.

This separates grant and constraint responsibilities. How filters target and compose
across conduits remains for review. Removing a constraint may expose an underlying
existing grant again; it does not create a new grant. Resource access and session
exchange share the conduit concept without granting one another's permissions implicitly.

Resource selectors support IDs, named ownership contexts, metadata predicates, and
path patterns where integrations support them. Pool membership also supports explicit
references and label selectors; membership does not authorize access. Resource matching
and authorized conduit/filter/role-limit changes apply live to subsequent managed
requests, with effective changes recorded and shown. They do not change model/account defaults or automatically inject context.

If an access-rule change removes access that an active isolated execution cannot revoke live,
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

V0 supports direct live read-only shares between contexts, managed and revocable by
authorized share managers. The source keeps its ID and owner; recipients select incoming
shares through their effective access selections and see current content within their effective access.
Revoking a share removes that grant on subsequent managed requests; independent grants
remain valid. This supports selective reuse without delegated grant chains.

Update rights through shares, pinned shares, onward delegation, and independent publication
are deferred. Revision-pinned record references remain supported and do not grant access
themselves. Agents can edit common pool knowledge through their own scoped update
grants. The table defines sharing semantics as each capability is introduced.

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

Registration configures an MCP client host to launch a server command or contact an endpoint;
authentication is a separate concern. [MCP's architecture](https://modelcontextprotocol.io/docs/2026-07-28/learn/architecture)
supports multiple clients of an HTTP service, while stdio usually connects a client to
its launched subprocess. Reusing a definition or endpoint does not pool live clients.
Multiple server processes need a shared backend to expose common durable state.

[Pi's native MCP support](https://github.com/earendil-works/pi/blob/4a080252f06caf63a525d7873c5d94d3c23887e0/packages/coding-agent/docs/mcp.md)
supports stdio/Streamable HTTP, user/project configuration, and dynamic extension
registration. It is a reference for Conduit's own runtime clients, not functionality
provided by importing Pi's frontend. Tool exposure/discovery and resource access are
distinct from Conduit's authorization rules; nested calls, inherited configuration,
and resource credentials need explicit handling.

[Codex](https://learn.chatgpt.com/docs/extend/mcp?surface=cli) stores registrations in
user/project TOML; [Claude Code](https://code.claude.com/docs/en/mcp) supports persistent
registrations and launch-specific `--mcp-config` with `--strict-mcp-config`.
[Codex configuration overrides](https://learn.chatgpt.com/docs/config-file/config-basic#configuration-precedence)
can supply launch values, but are not by themselves an isolated configuration source.
Generate native launch configuration from effective access without
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

Conduit's session, ownership, conduit and filter semantics are application contracts.
Bind requests to effective session identity/access; do not equate an MCP connection
with a Conduit session. Protocol authorization does not supply Conduit's complete
access policy.

Candidate mappings are tools for memory search/writes, resources for readable records,
and prompts for reusable instructions. Exact schemas and the protocol/extension baseline
must be verified with the selected hosts. Native BYOS remains usable regardless of
optional MCP skill-discovery support. No host/SDK compatibility has been tested.

## Observation and recovery

Record and attribute available token usage across agents, preserve available transcripts,
and associate observations with session/execution identity. Conduit's own runtime can
observe the provider/tool work it dispatches; provider-reported usage and external-agent
telemetry still have limits. Comprehensive interception, external subagent coverage and
user hooks remain broader goals. Exact cost accounting is deferred. Frontend progress
and optional trace exports must not be the only retained evidence of completed work.

V0 usage observations identify their execution/account and reporting source, distinguishing
reported counts, estimates, unknown data, and observed zero. Sum only observations known
not to overlap. Keep parent/subagent figures separate when overlap cannot be resolved,
so totals do not silently double-count usage. Missing native telemetry remains a visible
integration gap. Adapter validation must establish available attribution and aggregation
coverage; the common contract does not imply exhaustive collection.

For an open session after an unexpected interruption, reconnect to its existing execution
when the integration verifies that it is still running and supports reconnection. A
confirmed stopped execution requires explicit resume before starting a replacement.
Preserve the logical session and workspace; report unknown execution status without
launching another execution. Recovery restores supported continuity without automatically
starting new agent work. This does not change explicit pause/reopen semantics.

For operations Conduit controls, retries reuse the same operation ID and return a
recorded result when available, subject to current access. Distinguish confirmed effects,
failures, and unknown outcomes; a timeout alone does not establish failure. Automatically
retry only when repetition is known safe. After an uncertain write, check local/provider
state before reissuing it; unresolved outcomes remain unknown. Agents may reconcile
within their permissions without mandatory human approval.

This avoids repeating completed work without claiming universal duplicate prevention.
External integrations promise only the guarantees they support; native operations outside
Conduit's control remain outside this contract. A purged or unavailable operation result
does not prove that the operation never ran; the same reconciliation rule still applies.

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
- [Pi research report](RESEARCH_PI.md): frontend-only feasibility, detailed Durable
  recovery/context/storage mechanisms, proposed Rust runtime boundaries and validation.
- Local `jxp-skills` and `pstack-pi` checkouts inform usage accounting and native
  integration. Their storage, deployment, and skill-package designs are not Conduit
  requirements. Communication references, including `agentwatch`, are in REVIEW.md.
