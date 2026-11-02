# Conduit design contracts

Updated: 2026-11-02. Current working design; implementation
has not started. [SPEC.md](SPEC.md) summarizes the model; [REVIEW.md](REVIEW.md) records the
conceptual closure review and deferred authoring boundary. The [YAML walkthrough](../experiments/001-shared-access/README.md)
proposes concrete syntax. Research supplies supporting evidence, not extra requirements.

Prefer minimal bookkeeping: retain authoritative configuration, identity, authority and necessary
operation evidence; derive views rather than maintain parallel editable descriptions. Shared
contracts need not impose the same lifecycle, process model or implementation on every type.

**Implementation priority:** plugin and template authoring are central to the protocol and the
first implementation's success. Evaluate runtime choices by whether an author can define a small
extension, compose it into templates, establish instances and inspect their behavior without
modifying the core. Existing infrastructure can implement these boundaries; early efficiency is
secondary to correct, enforced contracts and a usable authoring path. This direction does not
approve particular SDKs, plugin packaging, deployment topology or schema spellings. Material changes
remain collaborative in the conceptual review; concrete authoring and implementation choices return
in later PRs.

**Accepted authoring direction:** start with extremely lightweight Conduit support and author
real plugins against the public contracts to validate the approach and speed development.
Prioritize the protocol and extension structure over a library of helpers. Existing protocol
libraries and small adapters may supply plumbing; richer SDKs can grow from repeated needs.
Plugins still supply their applicable behavior, effects and recovery semantics. This settles
implementation priority, not package syntax, supported languages, transports or process topology.

**Plugin publication:** publish inspectable contracts with their implementation bindings. Conduit can
inspect exports and validate typed configuration/graph bindings without starting plugin code;
actual availability, authority and behavioral conformance still require their respective checks.
Provider-native catalogs can remain dynamic behind an integration's published contract.
The published, language-neutral contract is authoritative for a plugin's public API and behavior;
implementations conform to it. Authors may handwrite or generate contracts, then explicitly review
and version the released definitions. Internal code changes do not implicitly change the public
contract. Schemas describe valid data; effects, recovery and lifecycle guarantees remain part of
the behavioral contract. Exact syntax, package layout and generation tooling are deferred to a later
authoring PR.

**Primary access model:** establish capabilities and service authentication through a Pool once,
then let authorized Agents use them through individually restricted Conduits. V0 Interfaces
provide unrestricted local administration; differentiated Interface permissions are deferred.
Sharing service access is the normal path, not an optional optimization of per-Agent logins.

## Contract map

The accepted rules are organized by responsibility. The topic files are the detailed design;
this index and SPEC summarize them. They are not competing specifications or a new decision ledger.
The split preserves the existing rules and their heading anchors; Git retains the earlier layout.

| Topic | Read it for |
| --- | --- |
| [Model](design/MODEL.md) | Five families, extension concepts, shared policy, instance authority, Interfaces and observability. |
| [Lifecycle](design/LIFECYCLE.md) | Templates, setup graphs, instance runtimes, supported controls and recovery. |
| [Configuration](design/CONFIGURATION.md) | Typed YAML, stable identity, references, checked edits, exports and implementation retention. |
| [Agents](design/AGENT.md) | Activation, model accounts, native/isolated modes, execution, continuity and observation. |
| [Resources](design/RESOURCES.md) | Unitary capabilities, type groups, memory, derived content and workspace rules. |
| [Pools](design/POOLS.md) | Explicit composition/provisioning, ownership, shared authentication and retirement. |
| [Conduits](design/CONDUITS.md) | Endpoint responsibilities, composable requirements, participation, grants, routing and communication. |

## Review boundary

The conceptual audit found three closure topics in [REVIEW](REVIEW.md): conformance scope,
authority across dependent calls, and lifecycle guarantees across connected objects. These are
review topics. C1 now accepts [declared-capability conformance](design/CONFIGURATION.md#declared-capability-conformance);
and the independent-client/plugin milestone. C2 accepts Resource dependency grants with applicable
caller restrictions. C3 retirement guarantees remain open. Already accepted behavior stays in force.
The [collaborator comparison](proposals/COLLABORATOR_REVIEW.md) identifies which proposed changes
align, which need adaptation and which remain separate future work.

Specific plugin/package fields, graph bindings and schema choices are parked in the
[authoring proposal](proposals/authoring-syntax/README.md) for a later PR. Runtime arrangements
remain in the [implementation proposal](proposals/implementation-v0/README.md). Neither a sample
YAML packet nor an implementation convenience may silently redefine a contract.
