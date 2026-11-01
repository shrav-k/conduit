# Implementation review: decisions needed

Status: the initial lightweight authoring direction is **accepted** below; the seven grouped
items still contain unresolved implementation choices. Other recommendations remain proposed.
Read the [architecture](ARCHITECTURE.md) for the connected system and
[contracts](CONTRACTS.md) for details. This queue concerns implementation boundaries; it does not
reopen the five families, Pool ownership, fixed definitions, shared authentication or typed filters.

## Review priority: plugin and template authors

The user has clarified that these rulings remain collaborative. Do not turn the recommendations
into defaults or close the queue autonomously. Easy, expressive plugin and template authoring is
the primary acceptance concern; deployment and efficiency decisions support that goal. Existing
infrastructure may implement the contracts without becoming part of the public authoring model.

Use these three concrete author journeys to review **I3 and I7 first**:

1. **Compose without new code:** use existing filesystem, authentication and MCP capabilities in
   a reusable Pool/Agent template, with explicit bindings, setup and permitted controls.
2. **Add a small operation:** supply a typed handler, such as a checksum operation, and reference
   it from a graph without writing unrelated service lifecycle machinery.
3. **Add a managed capability:** supply a new Resource type with setup, ongoing behavior,
   observation and supported changes, then instantiate and connect it using the same framework.

For each journey, identify what the author writes, what reusable tooling supplies, what the
protocol guarantees, and how it is tested. Keep the accepted typed contracts, finite graphs,
instance-owned lifetimes and fixed implementation bindings; their precise authoring surface is
still to be settled. Template-only composition should not require inventing a new plugin.

**Accepted authoring direction:** begin with extremely lightweight support from Conduit, with
most integration behavior authored as plugins to test the approach and speed development. The
priority is the public protocol and extension structure, not building a library of helpers.
Use existing libraries and minimal adapters where useful; richer support can grow from actual
plugin-authoring needs. Plugins remain responsible for applicable effects/recovery behavior.
This does not mandate a comprehensive helper runner or require every author to hand-write a
protocol server. The [walkthrough](AUTHORING.md) illustrates compatible authoring paths and the
extra responsibilities of effectful operations and managed capabilities.

**Still open within I3/I7:** the package/contract deliverables, wire profile, implementation
selection and concrete template/schema conventions. File layouts, fragments and code remain
illustrations, not accepted APIs. No language, process layout or transport is selected by the
authoring-direction ruling. Review these concrete boundaries next rather than reopening the
priority between framework helpers and real plugin implementations.

**Current discussion:** publish an inspectable contract bundle with each plugin implementation,
or discover descriptors by running the plugin and retain them afterward. The user requested a
[concrete filesystem package](AUTHORING.md#a-concrete-package-to-review) before ruling. The
example distinguishes package metadata, schemas, behavioral contracts and executable code;
provider-native dynamic catalogs remain separate. Bundle structure and schema source of truth
are not approved by the accepted lightweight-support direction.

Then review I1's execution/authority boundary against those authoring examples, I2's demonstration
sequence, and I4–I6's persistence/custody/evidence support. Preserve the existing IDs; this is a
change of review order, not another decision ledger.

## Pending choices

| Item | Decision | Recommendation |
| --- | --- | --- |
| I1 | Independent execution, authority and outage behavior | Examine shared management with independent execution services alongside independently authoritative object services; no host arrangement accepted. |
| I2 | First real Agent path and initial platform | macOS-first native adapters for subscription verification, followed by the owned Rust loop; thin custom terminal remains a deliverable. |
| I3 | Public plugin boundary and initial distribution | Versioned executable workers, MCP-first lifecycle profile, first-party/local packages; Wasm later. |
| I4 | Authoritative persistence | SQLite for local control/evidence; Resource backends remain replaceable. |
| I5 | Credential custody | OS-backed Conduit service secrets, native-runtime-owned Agent authentication; no implicit plaintext fallback. |
| I6 | Default observability payload capture | Durable metadata/recovery inputs plus explicitly classified content capture; no blanket full payload logging. |
| I7 | Concrete authoring/schema conventions | Uniform type fields, owner implementation plus named plugin aliases, reviewed JSON Schema bundles. |

I3/I7 establish what an extension author writes and can rely on. I1/I2 determine how to execute
and demonstrate those contracts; I4–I6 determine their retained state and evidence. Numbers for
queue sizes, internal helper names and performance tuning need tests, not separate ruling queues.

## I1 — Independent execution and management

**Status:** explicitly unaccepted. The user wants more independent objects, eventual placement
across systems, Pool Resources on different systems, and network Conduits. A single host owning
every execution is no longer this packet's default recommendation. Pool ownership is logical;
it does not specify an execution machine. The [research comparison](../../research/DISTRIBUTED_RUNTIME.md)
grounds the alternatives in Tilt, Docker/containerd, Kubernetes, Rust systems and papers.

**Recommendation for discussion:** separate definition/management authority, execution custody,
capability traffic and Interface aggregation. Examine independent execution services that can
supervise multiple objects; a common registry could initially coordinate them without being
their process parent or the mandatory transit point for every call. Also examine independently
authoritative object services before settling the registry boundary. These are implementation
roles, not new object families or a required daemon per object/Pool.

**Example:** one Euphony Pool owns a laptop worktree, a Linear/authentication integration on a
service machine and search on another machine. Two Agents connect to those Resources through
Conduits. Losing management visibility does not itself mean all those processes have stopped.
Losing Linear affects Linear access; it does not end the Pool or unrelated work. The Interface
must show unavailable/stale observations and reconcile them when connectivity returns.

**Resolve together, with examples rather than field-by-field rulings:**

1. Who owns active execution lifetimes: independent execution services, independently authoritative
   object services, or the previously proposed shared host as a deliberately limited first deployment?
2. Where is configuration/permission authority: one registry with distributed execution first,
   or independent authorities from the outset? Physical distribution alone does not decide this.
3. What can continue during a partition, and what must wait for current authority? Keeping a
   process alive is distinct from admitting a new protected effect. Offline grants would need an
   explicit freshness/withdrawal contract; they cannot silently weaken current-access rules.

**Proposed evidence:** extend experiment 002 to two execution processes on one machine, with
separately faulted management, worker, capability and observation links. Test continuing owned
activity, blocked unauthorized/uncertain effects, retained outcomes and verified reattachment.
Two-machine validation and production remote support can follow; whether they are required for
the first milestone is still open. No crash-survival guarantee is accepted by this proposal.

I3 worker lifetime and I4 persistence ownership depend on this ruling. Frontend detachment,
explicit resume of stopped Agent work, current authority checks and no duplicate unknown work
remain accepted. This review does not reopen those behaviors.

## I2 — First real Agent path and supported platform

**Recommendation:** validate on macOS first, establish the synthetic Rust supervisor and terminal
inspection surface, then prioritize native Claude/Codex adapters for the subscription/account
experiment. Build the Conduit-owned provider/tool loop as a distinct subsequent milestone. Keep
portable Rust core tests suitable for Linux; do not claim Windows/native isolation support before
its process/credential/environment adapters exist.

**Example:** two Claude and two Codex subscriptions can exercise shared MCP and communication
through native drivers while Conduit owns identities, connections, history Resources and evidence.
That verifies the account goal; it does not pretend their native agent loops are Conduit's loop.
The later owned-loop driver supplies Rust-controlled context, provider dispatch and tool iteration.

**Alternative:** implement one direct model provider and the owned loop first, then native accounts.
This demonstrates the desired Rust runtime earlier but may initially require a different supported
access mechanism and does not establish subscription-account compatibility.

**Ruling needed:** delivery order and macOS-first validation scope. The accepted end state still
includes Conduit's own runtime and a replaceable Pi-inspired terminal. No proposal adopts Pi's loop.

## I3 — Plugin execution and distribution

**Recommendation:** use exact-version executable workers for externally supplied implementation
code. Standard MCP tools/resources and optional features carry compatible exchanges; document a
small Conduit worker profile for owner-bound lifecycle/recovery. Internal trusted Rust traits
remain an implementation convenience. Start with first-party bundled artifacts and explicit local
package installation; defer a marketplace and arbitrary-language dependency resolver.

**Example:** a Pool-owned service Resource keeps worker A while a new application release ships
worker B. A replacement Resource can select B and explicitly reuse compatible backing. Installing
B does not modify A's behavior. A standard remote MCP service remains usable without pretending
its server implements the Conduit worker lifecycle profile.

**Alternatives:** a dedicated binary RPC for workers may make lifecycle methods simpler but adds
another public protocol; in-process-only plugins reduce initial process plumbing but complicate
exact implementation retention and failure containment; Wasm-first adds a host-capability and
packaging system before proving the required native integrations.

**Ruling needed:** adopt MCP-first executable workers as the first extension target, or prefer a
different boundary. Exact artifact pinning and replacement on upgrade are already accepted. The
experiment must test whether the MCP profile stays small before treating its method set as stable.

## I4 — Local transactional authority

**Recommendation to evaluate after I1:** SQLite for each chosen local durable authority, accessed
through a bounded store thread with WAL/FULL commits. Registry revisions, target execution
outcomes and aggregated observations need explicit ownership; an independent execution service
cannot rely exclusively on a vanished management process's in-memory state. Resource backends
remain separate. This is not a proposal to share one SQLite file across machines or to permit
multiple writable copies of an instance spec.

**Example:** Xerj is offline but the Interface can still show that a search Resource is unavailable,
what setup completed and which control is uncertain. Rebuilding its search index does not change
an Agent's ID or erase a delegation. A stopped exporter does not lose local operation outcomes.

**Alternative:** make the control store itself replaceable through an external plugin from day one.
That requires a stronger transactional/concurrency/bootstrap contract and could make repair depend
on the failed plugin. JSONL-only storage is simpler to inspect but moves transaction/index/compaction
responsibilities into our code.

**Ruling needed:** choose local persistence after settling authority and execution separation.
Which facts commit together locally, and which use recoverable inter-service exchanges, must be
explicit. This does not make SQLite a required backend for every Resource or settle federation.

## I5 — Credential custody and account compatibility

**Recommendation:** explicit OS credential-store support for secrets Conduit itself owns, with
in-memory fake custody for tests. An unavailable/locked store reports a wait/error instead of
falling back silently. Native Agent adapters keep authentication under their supported native
runtime ownership and prove separate-account behavior. Pool service identity remains separate.

**Example:** one authentication Resource supplies appropriate service bindings to shared integrations.
Agents receive permitted capability use, not a token copied into each private profile. Meanwhile,
each native model account's refresh/logout is tested against the other account and against separate
Agent histories. This cannot be proved by choosing different directory names alone.

**Alternative:** begin with explicitly protected local credential files, postponing OS-store
integration. That simplifies automation/headless setups but changes secret-at-rest expectations and
requires an explicit storage and unlock policy. It must not be an undocumented fallback.

**Ruling needed:** secret-store default and acceptable first-platform enrollment experience. This
does not require another Conduit login or change native runtime authentication ownership.

## I6 — What observability captures by default

**Recommendation:** always retain operation identity, actor/target/path, effect outcomes, coverage
and the safely retained inputs needed for promised recovery. Retain message/history content under
its Resource contract. Capture other full tool/model payloads only under an explicit operation/type
recording policy, with content references, bounds and sensitive-field handling. Show missing,
truncated or deliberately uncaptured content clearly. Never log credentials by default.

**Example:** the Interface can prove a Linear call was admitted, which Agent/account/path performed
it and whether application was confirmed. It may have a permitted stored response, an excerpt or
only a reference depending on the declared policy. A missing full response is not presented as
complete transcript coverage. If omitted input prevents replay, that limitation is recorded.

**Alternative:** full managed tool/model payload capture by default, with explicit sensitive-field
exceptions. This improves debugging/transcript reconstruction but retains more private content and
requires reliable payload classification, size limits and scoped access from the outset.

**Ruling needed:** capture default. Retention-until-explicit-purge for captured evidence remains
accepted; this decides what is captured, not a new automatic-expiry rule. Interface observability
remains a first-class delivery requirement under either option.

## I7 — Authoring conventions and multi-plugin graphs

**Recommendation:** retain the accepted envelope and propose `conduit/v0alpha1` as its initial
identifier. Apply group/name/version type references uniformly to the five families. Keep
`spec.implementation` for the type's owner and add a `spec.plugins` alias map for other exact
implementation dependencies; graph exports use `{plugin, name}`. Keep schema bundles reviewed and
language-neutral, with Rust schema generation used as assistance rather than silent authority.

**Example:** a Resource can use its owner's generic connection preparation, an authentication
export from another package and a database migration export, all explicitly bound. Those code
aliases do not create Resources, own credentials or bypass Conduits. Instance expansion resolves
the graph/controls and records code selection independently of later template changes.

**Alternative:** use one uniform `implementations` map, with an explicit alias for the type's owner.
That is more uniform but adds indirection to every simple template. A Rust-derived public schema
source of truth is also possible, provided release checks prevent accidental language changes.

**Ruling needed:** choose the authoring arrangement and schema source of truth as a bundle, then
review amended YAML examples. Parser scalar edge cases, numeric precision limits, schema-default
coverage and negotiated worker-feature details are implementation/conformance work; a failing
example should surface a concrete decision rather than another speculative questionnaire.

## Decisions not requested again

We retain the five families; Pool-only Resource ownership; templates as complete recipes; finite
typed graphs; instance-owned runtimes; current authority checks; native tool preservation; fixed
instance definitions and implementations; supported controls; shared service auth; private Agent
context; retained communication with independent readers; no default automatic wake; and explicit
cleanup/purge. None is being quietly traded away for convenience in a crate or framework.

## After review

Record rulings here in place, fold any changed accepted behavior into DESIGN, and update the
concrete experiment packet before writing code against its new syntax. Implement M0–M2 first.
Use the runtime experiment to resolve engineering uncertainties, then execute the selected real
Agent path. This queue should shrink as choices settle, not become another permanent micro-decision ledger.
