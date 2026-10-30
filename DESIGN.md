# Conduit architecture and contracts

Updated: 2026-10-30. Read [SPEC.md](SPEC.md) for vocabulary and expected behavior.
This describes current boundaries; [REVIEW.md](REVIEW.md) contains unresolved choices.

## Ownership and extension boundaries

The native Rust core coordinates identity, sessions, shared records, room history,
and managed resource access. Integrations preserve provider-native behavior. These
are responsibilities, not a required collection of processes or crates.

| Owner | Responsibility |
| --- | --- |
| Conduit | Local roles/connections, logical sessions, scopes, shared knowledge/checkpoints, room communication, and available observations. |
| Native agent | Planning, execution loop, private working context, native skills/tools, and native subagents. |
| Resource or tracker provider | Source content and native operations; Linear issue fields, assignments, status, and workflow. |
| Storage/search plugin | Persistence and retrieval under Conduit's supported record/access contracts, with explicit capability and freshness limits. |
| Frontend/runtime integration | Presentation, launch/attachment, account selection, input/control, and supported lifecycle/events. |

Plugins may integrate models, runtimes, frontends, resources, trackers, storage/search,
and room transports. A connection is a reusable local instance of such an integration.
Scopes select shared connections and grant their permitted tools/resources; roles
supply execution settings, native skills, and optional permission ceilings. A role
does not need a second list of shared connections. Definition alone grants no access.

Resolve the selected scopes against role limits for each session, then let its runtime
integration prepare native MCP setup. Keep host-specific transport, configuration, or
authentication handling in that integration; report unsupported capabilities. A model
change alone need not create a new logical connection, while a runtime change can
require different native setup. Shared definitions do not merge permissions or imply
that every host can reuse the same authenticated client state.

Xerj remains a candidate behind a replaceable storage/search plugin. No database,
plugin ABI, transport, process model, or hosting arrangement is selected. Authoritative
records, observations of external state, and search projections have different meanings.

## Sessions and workspaces

One logical session can span multiple native executions. It has at most one active
primary agent; independently coordinated workers have separate sessions. Native
subagents may remain inside their parent's execution. The logical session identifies
a room participant; each message identifies its producing execution.

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
not fork files, and a worktree does not imply isolation. The binding among task,
room, issue, and workspace, concurrent editing rules, and cleanup remain open.

## Rooms and tracker boundaries

Rooms store append-only communication with durable shared history and independent
session read positions. Participants may retain local copies. A room has no shared
editable working-memory document; separately scoped memory/checkpoints can be linked
or quoted. Retention is independent of immutability and remains to be selected.

Posts do not automatically load their contents into every participant's context.
The default is a next-normal-turn unread notice where supported. Optional automatic
wake hooks are future work. Room membership, cursor initialization/advancement,
addressing, retries, and delivery acknowledgements need concrete contracts.

Keep room identity independent of the linked tracker issue. Initial tests can omit
tracking; the eventual linkage requirement has no selected transition milestone or
cardinality. Provider-owned tracking is sufficient, initially Linear. Synchronization
may use provider-native operations without a full replica. Room posts do not by
themselves authorize publication as external issue comments; exchange rules remain open.

## Memory and record lifecycle

Conduit retains knowledge, checkpoints, room messages, usage observations, and artifact
references. Plugins may add namespaced record kinds. An internal work-item engine is
optional. Type-specific payloads share small metadata for identity, ownership, revision,
provenance, and links; external provider objects need not adopt this schema.

- **Identity:** stable record IDs survive renames and moves. Each record has one owning
  context, meaning a task, project, or personal collection used as its ownership anchor.
  This is distinct from an agent's working context. References can be live or pinned
  to a revision; unavailable pinned content is never silently replaced.
- **Updates:** mutable kinds create immutable revisions. A stale base revision returns
  a conflict; agents may reconcile and retry without mandatory human approval. Room
  posts remain immutable and use follow-up messages for corrections.
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
are deferred. Retrieval/search operations and budgets remain open.

## Access and sharing

Effective access is the union of selected scope grants, constrained by declared role
limits and explicit exclusions. Missing role limits add no grants. Permissions name
operations per resource type; convenience bundles expand into visible grants. Permitted
operations do not require an extra human confirmation merely because permissions exist.

Selectors support IDs, named ownership contexts, simple filters, and path patterns
where integrations support them. Resource matching and authorized scope/role-limit
changes apply live to subsequent managed requests, with effective changes recorded
and shown. They do not change model/account defaults or automatically inject context.

Selecting a context covers directly owned records by default. Descendants and incoming
shares need explicit inclusion, which presets can provide. Ordinary links grant no
access. Unsupported/restricted bindings remain unavailable while other usable access
proceeds; integrations must not silently drop restrictions.

Check whole-record access using the recipient's current permissions immediately before
managed delivery, including queued messages/handoffs, retries, replays, and retrieval.
Mixed-access content is withheld as a whole; narrower delivery requires a separately
authorized extract. Apply the same access rules to search, caches, transcripts, and
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

Preserve native provider tool names, descriptions, schemas, results, and workflows
wherever possible; identify necessary adaptations. The target is interception of all
outgoing MCP calls from managed role sessions, with attribution and eventual permission
checks that allow, limit, or block calls. This does not require one combined endpoint:
a common gateway or multiple provider endpoints can provide mediation.

V0 need not implement comprehensive interception, upstream policy enforcement, or
sandboxing. Keep a path to those capabilities and report actual coverage. Basic access
to Conduit-managed resources remains core. MCP interception is the first boundary;
future tiers may cover native tools, subprocesses, files, and network interactions.
Each tier must state observed activity, enforced restrictions, and uncovered paths.
Preventing bypass requires more than configuring an agent's MCP connections.

Conduit's sessions, ownership, scope, and room semantics are application contracts.
Bind requests to effective session identity/access; do not equate an MCP connection
with a Conduit session. Resources/notifications do not provide durable room history,
and protocol authorization does not supply Conduit's complete access policy.

Candidate mappings are tools for room posts/reads and memory search/writes, resources
for readable records/history, prompts for reusable instructions, and notifications
for supported unread signaling. Exact schemas and the protocol/extension baseline
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
- Local `agentwatch`, `jxp-skills`, and `pstack-pi` checkouts informed room history,
  independent consumers, usage accounting, work coordination, and native integration.
  Their storage, deployment, and skill-package designs are not Conduit requirements.
