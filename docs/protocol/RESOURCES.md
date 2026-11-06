# Resource contracts

> **Superseded. Nothing in this file binds.** It predates rulings 16 to 37 of the
> [plan audit](../AUDIT.md#ruling-log). The specification in [`spec/`](../../spec/README.md)
> governs, and where this file differs from it, this file is wrong.
>
> It is kept for two reasons only. The first is as source material for type contracts that
> are still to be written: the directory, worktree, memory and workspace types. The second is
> so that links into it resolve.
>
> A behavior that is stated here and not in `spec/` is a gap in `spec/`, not a rule. The
> [audit of the documents outside `spec/`](../proposals/docs-audit/README.md) found the known
> ones, and its [report](../proposals/docs-audit/reader-older-text.md) lists them under "Gaps in
> the specification". The words here are the older ones, among them grant, plugin, driver,
> handler, provision and `form`; [`spec/definitions.md`](../../spec/definitions.md) has the
> current vocabulary.

Scope: superseded working text, not a contract. No engine is held to it. Where the text below
says accepted, settled, current or v0, it means the design as it stood when this file was last
updated, before rulings 16 to 37.

Once one of the design topics that the [pointer page](README.md) now lists. Last updated:
2026-11-02. The [conceptual review](../archive/reviews/CONCEPTUAL.md) and the other reviews
linked below are history.

## Resources

A Resource template describes how to create or attach data/capabilities through a typed
contract. It is reusable configuration, not a deployed capability, and need not belong to a
Pool merely to exist. Creating or attaching a Resource instance is exclusively a Pool operation.
Every instance has exactly one owning Pool; one instance cannot be a member of several Pools.

The Resource template also declares allowed configuration changes and their parameters, using
the common control contract within its type's supported behavior. Its exact field layout remains
open; the previously settled type/config/connection fields are:

Resource specs use these fields in both template and instance forms:

| Field | Contract |
| --- | --- |
| `spec.type` | A type-contract reference with separate `group`, `name` and `version` fields. This is distinct from a registry object reference using kind/form/name. |
| `spec.config` | Type-specific settings validated for the selected type and form. Templates declare establishment intent; instances retain initial and then confirmed settings in one map under the shared configuration contract. Exact fields are schema work. Unknown fields are rejected except in schema-declared open maps. |
| `spec.conduits` | The consumer's Conduit declarations. Their concrete entry syntax is proposed in the walkthrough. |

Resource types have logical groups independent of installation packages. A plugin package may
supply types from several groups; changing implementation packaging need not rename contracts.
Group and type name are separate fields, not a slash-delimited identifier. Use `spec.type.group`
for the group, `spec.type.name` for the type within it, and `spec.type.version` for its contract
version. The group catalog is extensible; it is not a fixed list of Resource kinds.

The first working catalog uses four core groups:

| Group | Responsibility |
| --- | --- |
| `filesystem` | Expose or manage filesystem trees. |
| `storage` | Persist data with defined operations. |
| `search` | Retrieve relevant results from configured sources. |
| `integration` | Expose configured native service/tool interfaces. |

These group boundaries are accepted. Within `filesystem`, `local-directory` and `git-worktree`
are separate types: both expose directory access, while the latter also has Git-specific
creation, attachment and cleanup obligations. This selects contract boundaries, not a
requirement to implement both immediately. Additional types and their detailed schemas are extensible.
Keep an additional agent-communication group under consideration for direct messaging,
multi-participant conversations and shared context. Agent-specific contracts can justify a
distinct group even when they use familiar storage/integration mechanisms. Its name, additional types
and shared-context capabilities remain open. Pool-owned content and Conduit relationship
responsibilities are settled; the first retained-history capability is defined under [communication](CONDUITS.md#communication-contract);
that does not require every communication Resource to use that shape.

Conflicting definitions for a contract identity/version must be reported rather than
silently overwritten. Public contract ownership/governance, allowed identifiers and version labels
remain drafting/future work. Groups and type contracts add no registered object family, assign no Pool ownership and
grant no access. [Resource typing](../research/RESOURCES.md) retains the supporting investigation,
candidate types and proposed interface boundaries; those proposals are not built-in requirements.

Resource type contracts are versioned independently of the common object `apiVersion`.
The common format can remain unchanged while a type's configuration/behavior contract evolves.
Plugin implementation releases are separate from both versions; a plugin release number does
not substitute for either contract version. The retained implementation binding and replacement
rule apply; identifier spelling and concrete compatibility checks remain to be drafted.

**Compatibility boundary:** required connections name a versioned capability contract,
describing the operations and behavioral guarantees a consumer needs. Different Resource types
may fulfill that same contract; consumers need not require a particular concrete Resource type.
A Resource type still defines configuration, establishment and lifecycle. A capability contract
defines the promised consumer-facing behavior; it is not an Interface instance or a sixth family.

For example, local-directory and git-worktree could both fulfill a directory-access contract while
retaining distinct creation and cleanup rules. This is a compatibility example, not a finalized
contract name or guarantee of identical filesystem behavior. Matching requires the promised
semantics, not merely a shared group, operation name or data shape. Pool target selection remains
explicit; provider/account constraints, current permissions and readiness remain separate checks.
Capability identifiers, compatible-version rules, conformance evidence and any supported adaptation
remain open. A matching declaration is not proof of actual implementation or current availability.

The same template may create distinct instances in multiple Pools. A contract supports new
backing data, attachment to existing backing data, or both explicitly. Different instances
may therefore reach the same native target without sharing registry ownership. Creating a
new instance does not prove that files, credentials, quotas or remote data were copied.

Resources should be as simple and unitary as possible: one responsibility with its related
operations. A storage Resource may read/write/delete records, a search Resource runs queries,
and an MCP-access Resource handles its configured integration. Keep those responsibilities as
separate Resources by default and group them in Pools, including when one provider supplies
all three. Separate registrations do not require separate backend processes or copied data.
Access between Resource instances is expressed through Conduits; exact configuration syntax,
connection creation and type schemas remain open.

A repository, storage collection, search capability, skill, document or provider access is an
example, not a closed taxonomy. Related operations stay within their Resource; each operation
or native tool does not require a separate Resource merely because it is independently callable.
Native files, tickets and records likewise need not each become a registry object.
Discovery uses defined fields and relationships, never granting authority or transferring ownership.

Resource instances record enough identity/contract information to resolve their source and
supported operations. Provider-owned data retains native contracts; attaching it does not
make Conduit the owner of the provider account or authorize deleting that backing data.

Resource templates and instances provide concrete protocol review cases. A local path exposed through a
symlink, an MCP connection and hybrid search over a database are contrasting examples, not
interchangeable implementations of a fixed lifecycle. Settle their typed configuration,
backing identity, operations and shared/per-consumer state before deciding Pool grouping.
A type contract defines supported semantics; a template supplies reusable establishment
configuration and declares the permitted configuration changes within that support.
Other Resource contracts can be developed against the [schema review criteria](../archive/reviews/CONCEPTUAL.md#drafting-and-deferred-work).

For a local folder, the Resource instance describes the configured folder attachment. Each
Agent's symlink is preparation state, not another Resource instance. Two Agents may use
the same Resource through different local paths; removing one prepared symlink leaves the
Resource, its owning Pool, backing folder and other Agent's view intact. Creating another
symlink does not copy the folder or isolate concurrent edits. Exact path placement and cleanup
timing remain open; other Resource types may require different consumer lifetimes.

The `local-directory` type defines folder access without Git lifecycle management. Registering
a checkout under that type does not imply managing its branches or worktree creation/removal.
The `git-worktree` type additionally represents its relationship to a Git repository and the
supported worktree-management operations. Both retain Pool ownership, Agent-specific
preparation and explicit backing-data cleanup rules.

In v0, `local-directory` declares establishment intent in its template spec: attach an existing
directory or create a new one through its Pool. Attachment never creates a missing path.
A fresh creation reports a conflict when its target already exists instead of silently adopting it. Registration
alone performs neither operation, and backing-data cleanup remains separate. Retries still use
recorded operation outcomes and the existing safe-retry rules; an uncertain outcome does not
justify blindly repeating creation. Instantiation reads this declared intent rather than asking
the caller to assemble a separate provisioning recipe. Exact configuration/transition syntax,
Git-worktree-specific modes and any shared directory interface remain open.

Local-directory templates may contain relative backing paths. Creation resolves them against
an explicit base and saves the resulting absolute host path in the complete instance spec.
For example, `./repo` with base `/work/euphony` becomes `/work/euphony/repo`. Later use does
not depend on the launching terminal's working directory or reinterpret the original relative
path. This is the backing path, distinct from each Agent's prepared path. Symlink resolution,
Agent-local placement and the exact configuration/operation fields remain open.

### Native resources and integrations

Users bring compatible skills/plugins through Pools and Conduits. User-level native
settings are selected explicitly; an existing setup can be selected once, but the user's
default agent profile is not automatically inherited. Mandatory host/organization policies
still apply. Selecting a package does not grant extra MCP or resource access.

Use ecosystem tooling for compatible skill/plugin installation and updates. Adapters may
invoke it on request and record source/version information and outcomes. Conduit does
not require a new skill format or package manager. Executable native extensions need a
compatible runtime adapter or port; report unsupported hooks. Agent activation instructions
control preparation of the selected resources.

Resource providers retain source content and operations. Linear/Jira integrations are ordinary
Pool-owned Resources accessed through Conduits; the provider owns its issue fields, assignments,
status and workflow. Message history is also a Resource. There is no special tracker abstraction,
required issue link or mandatory Linear/Jira integration in the core protocol. If an application
needs specialized message tracking or work associations, it can supply a custom Resource capability.
This does not introduce an internal ticket engine or make common observability optional.
Storage, search and graph backends are replaceable; Xerj is a candidate plugin backend.
Authoritative records, observations of external state and search projections are distinct.

<a id="v33"></a>

### Retained memory and evidence

Contexts, notebooks and similar memory surfaces can be ordinary Resources exposing plugin-backed
capabilities. Their templates define configuration, setup, ongoing behavior and supported controls;
their instances belong to one Pool and are accessed through Conduits. A context Resource can retain
its own data or expose data from other Resources through authorized dependency Conduits. Exposure
does not transfer the source records' ownership. Where only filtered access to an existing Resource
is needed, a Conduit can provide it without creating another Resource.

There is no universal core Context layer or record-owning-context hierarchy. Internal collections
and provider-specific organization belong to the relevant capability contracts. The record,
revision, provenance and sharing rules below describe the capabilities that provide those behaviors;
they do not require every Resource type to implement a common memory schema.

Knowledge, checkpoints, usage observations and artifact references share small record
metadata while retaining type-specific content and operations. Plugins may introduce
namespaced record kinds. Memory is exposed as Resources through Pools and Conduits.
Work and personal knowledge remain separate unless explicitly shared.

| Concern | Contract |
| --- | --- |
| Identity and ownership | Stable internal record IDs survive names/paths/external-key changes. A managed record has one authoritative owning Resource, which belongs to one Pool. A Resource exposing another Resource's records preserves their source identity and ownership. Provider-owned backing data retains its native ownership; an Agent's private working context remains separate. No generic owning-context object is required. |
| References | Support live and revision-pinned references. Missing pinned content is reported unavailable, never silently substituted with a newer revision. References alone grant no access. |
| Revision and concurrency | Each accepted mutable-record update creates an immutable revision. Updates name their base revision; stale writes conflict. Agents may reconcile and retry without mandatory human approval. |
| Provenance and time | Separate original source/author, revision writer and derivation links. Preserve local recording time separately from labelled source times. Unknown provenance remains unknown. |
| Lifecycle | V0 knowledge supports create, read/search, revision-checked update, archive/restore and explicit purge under operation permissions. Archive removes ordinary retrieval, not explicit authorized access. Purge removes retained content/history with a minimal deletion marker. |
| Retention | Records, revisions and captured history under Conduit's control remain until explicit purge; automatic expiry is deferred. Report native/backend limits outside that control. A missing payload is not a promise of recoverability. |
| Artifacts | References are searchable/linkable independently of their files. Removing an artifact record does not delete the referenced file or imply that a snapshot existed. |

Users and agents write knowledge explicitly. Classification is optional, entirely
user-defined and may use multiple dimensions/labels without advance declaration. Optional
definitions are ordinary accessible knowledge records referenced explicitly; matching
names do not bind automatically. Background extraction and classifier interfaces are deferred.

Memory search defaults to all memory permitted by the Agent's current Conduits and
limits. Queries may narrow by source Resource, kind, classification or other supported typed fields; narrowing cannot
expand access or implicitly search every repository/service. Results contain bounded
excerpts, record/revision references and permitted provenance. Full content requires an
explicit read. Configurable defaults/caps bound result count and total returned text;
agents can adjust within caps, and responses identify limits reached.

Partial search returns available results with an incomplete status, known coverage gaps
and backend-supplied freshness. Unknown coverage/freshness stays unknown. An unavailable
backend is not “no matches.” Reads, search, caches, transcripts and derived content use
current access checks and reveal only permitted diagnostic/provenance details.

### Sharing and derived content

V0 supports authorized, revocable, direct live read-only sharing of Resource-backed records
through Conduits. The source retains its ID and owning Resource. Access may be exposed directly
or through another Resource providing a shared-context capability; the latter uses authorized
dependency connections and preserves caller restrictions under the
[dependent-call contract](CONDUITS.md#authority-across-dependent-calls). Other independent grants
remain valid after one share is withdrawn. Common knowledge can be updated through separately
granted update rights. Pool ownership or a record reference alone creates no access. Sharing
content does not assign its source Resource to a second Pool or require a core context registry.

Share management is separate from read/update permission. A manager may grant only rights
it currently holds. Incoming-share views, collections and any traversal of their contents are
explicit behavior of a supporting capability and its authorized connections. Selecting a Resource
does not itself authorize following arbitrary references or traversing a generic context hierarchy.

Derived records and ordinary copies require read access to the record and all declared
source inputs. Copies retain those dependencies despite independent identity/content.
Unverifiable required permissions withhold content without deleting it. Missing source
payloads do not block stored derived content when current access remains verifiable;
historical provenance gaps alone do not block otherwise permitted use. Hidden source IDs,
titles, paths, URLs and previews are withheld with a generic notice.

The following preserve agreed extension semantics; they are not v0 delivery requirements:

| Extension | Contract |
| --- | --- |
| Richer shares | Pinned shares expose an exact revision; additional operations are explicit and bounded by current authority. Revision-pinned references remain usable in v0 without granting access. |
| Further grant issuance | Creating or changing another grant requires current authority for that administration. Issued grants are independent of issuer ancestry; there is no parent-grant cascade. Ordinary read access does not itself authorize further grants. |
| Grant lifetime | Any currently authorized administrator may withdraw a grant. Participation grants also end with their recipient participation or related Conduit. Issuer changes do not retract previous grants; current governing restrictions still apply. Explicitly withdrawn grants require authorized restoration/reissue. |
| Independent publication | A separate record may release source-access dependencies only with read/publication authority for each released restricted input and destination creation rights. Retain source/version provenance. The published record has its own audience/lifecycle and does not follow source changes or withdrawal. |
| Published edits | Edits using already-published content need ordinary update rights. New restricted material requires current read/publication authority for those inputs and destination update rights. Explicitly released attribution remains readable. |

### Workspace resources

A workspace Resource instance belongs to its Pool and is associated with a task or
experiment; Agents attach through permitted access. V0 supports existing folders/checkouts
and Conduit-managed Git worktrees, recording management status. Attachment preserves existing files and Git state and grants no deletion
authority. Concurrent independent writers use separate task-associated worktrees by default;
sequential handoffs can reuse a workspace. Shared editing is opt-in, without automatic
merge/file-lock guarantees. A worktree is not an isolation boundary.

Retain workspaces after Agent closure or task completion until explicit cleanup.
Cleanup of attached folders only unregisters them. Remove a managed worktree only when
no execution uses it and no unique local work would be lost; uncertainty preserves it
with an explanation. Exact creation/base/branch/path settings remain open.

