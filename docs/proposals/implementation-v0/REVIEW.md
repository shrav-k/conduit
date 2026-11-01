# Implementation review: decisions needed

Status: lightweight authoring support, contract publication, MCP, code-optional implementation
bindings, independent component access, management/execution separation and shared policy with
instance-owned state are **accepted** below; the seven grouped
items still contain unresolved implementation choices. Other recommendations remain proposed.
Read the [architecture](ARCHITECTURE.md) for the connected system and
[contracts](CONTRACTS.md) for details. This queue concerns implementation boundaries; it does not
reopen the five families, Pool ownership, fixed definitions, shared authentication or typed filters.

## Review priority: plugin and template authors

The user has clarified that these rulings remain collaborative. Do not turn the recommendations
into defaults or close the queue autonomously. Easy, expressive plugin and template authoring is
the primary acceptance concern; deployment and efficiency decisions support that goal. Existing
infrastructure may implement the contracts without becoming part of the public authoring model.

Use these three concrete author journeys to review **I3's conceptual extension boundary first**;
defer I7's detailed contract/template syntax until all conceptual questions are settled:

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

**Accepted publication boundary:** publish inspectable contracts with the implementation, so
Conduit can validate templates without starting plugin code. The
[filesystem example](AUTHORING.md#a-concrete-package-to-review) illustrates this boundary;
its filenames, fields and layout are not approved syntax. Published contracts do not certify
runtime readiness or implementation conformance. Provider-native dynamic catalogs remain separate.

**Accepted invocation boundary:** MCP is the public plugin protocol base, with a small versioned
Conduit profile for instance/operation context and lifecycle/recovery semantics. This does not
settle execution custody, exact profile bindings or transport, and does not require existing
provider MCP servers to implement the profile.

**Accepted implementation bindings and component access:** plugins publish contracts fulfilled
by authored code, existing local/remote MCP services, or both. No new plugin-specific executable
is required where compatible bindings and a shared adapter supply the behavior. Supported MCP
capabilities can be used independently while the Interface aggregates control/observations.
The [design-fit audit](../../REVIEW.md#design-fit-audit-code-optional-plugins-and-independent-mcp-access)
retains Pool ownership, current access, explicit lifecycle and required evidence. No process per
component, implicit discovery grant or invented provider guarantee follows. Ordinary endpoint
configuration can remain a Resource template; the [search example](AUTHORING.md#worked-comparison-configuring-search-versus-publishing-a-search-capability)
shows when a reusable contribution is useful. Syntax remains deferred.

**Accepted execution custody:** separate execution services supervise Conduit-managed Agents
and Resources; management coordinates configuration/control and the Interface presents the system.
Each executor may host multiple instances, including initially on the same machine. External MCP
services retain their own supervision. This adds no object family or process-per-object rule.

**Accepted authority:** shared registration/template and access-policy authority; one designated
service owns each instance's resolved definition, confirmed configuration/revision and operation
results. The Interface aggregates sources with freshness and gaps. A service can host many instances.

**Next discussion:** I1's outage behavior. Authority ownership is settled; it does not determine
which new effects are allowed when the relevant authorization service cannot be reached.

**Review order:** settle remaining conceptual plugin, execution, authority, demonstration,
persistence, credential and observation questions before detailed syntax. I7's identifiers,
field names, template bindings, package layout and schema-authoring conventions are deferred to
the final pass. File fragments and code remain illustrations. No language, process layout or
transport is selected by the authoring/publication/protocol rulings.

Then review I1's execution/authority boundary against those authoring examples, I2's demonstration
sequence, and I4–I6's persistence/custody/evidence support. Preserve the existing IDs; this is a
change of review order, not another decision ledger. Finish with the deferred I7 syntax review.

## Pending choices

| Item | Decision | Recommendation |
| --- | --- | --- |
| I1 | Outage admission and first independence proof | Separate execution and shared policy/instance-owned state accepted. Recommend retaining supervision/results while holding new effects whose current authority cannot be established; outage ruling remains open. |
| I2 | First real Agent path and initial platform | macOS-first native adapters for subscription verification, followed by the owned Rust loop; thin custom terminal remains a deliverable. |
| I3 | Plugin implementation execution and distribution | MCP, code-optional bindings and independent exposure accepted; execution packaging/distribution remain open after I1. |
| I4 | Authoritative persistence | SQLite for local control/evidence; Resource backends remain replaceable. |
| I5 | Credential custody | OS-backed Conduit service secrets, native-runtime-owned Agent authentication; no implicit plaintext fallback. |
| I6 | Default observability payload capture | Durable metadata/recovery inputs plus explicitly classified content capture; no blanket full payload logging. |
| I7 | Concrete authoring/schema conventions — review last | Uniform type fields, owner implementation plus named plugin aliases, reviewed JSON Schema bundles; proposed spellings remain deferred. |

I3 establishes the conceptual extension boundary. I1/I2 determine how to execute and demonstrate
those contracts; I4–I6 determine retained state and evidence. I7 finalizes their authoring syntax
after those conceptual rulings. Numbers for
queue sizes, internal helper names and performance tuning need tests, not separate ruling queues.

## I1 — Independent execution and management

**Accepted execution arrangement:** use separate management and execution services for the first
implementation. Execution services supervise Conduit-managed Agents and Resources; management
coordinates configuration/control; the Interface presents the system. Each execution service can
host many instances, and several can initially run on one machine. External MCP services retain
their own supervision. Pool ownership is logical and does not determine placement. These services
are implementation roles, not new object families or a daemon per object/Pool.

The [distributed-runtime comparison](../../research/DISTRIBUTED_RUNTIME.md) and
[cloud follow-up](../../research/CLOUD_AUTHORITY.md) supply the rationale and alternative examples.
Their shared-policy/instance-state arrangement is now accepted; outage, storage and remote
availability guarantees are not implied by that ruling.

**Accepted authority split:**

| Facts | Authoritative owner |
| --- | --- |
| Templates, registration/naming and shared access policy | Management authority |
| An instance's resolved definition, confirmed configuration/revision and operation results | One designated instance service; a service can host many instances |
| Backing domain data | The Resource backend/provider |
| Aggregate inspection | Interface views identify sources, freshness and gaps; no second writable authority |

Management can host passive instance authorities and executors active ones. State authority is
separate from a Resource's Pool owner and from the plugin code fulfilling a contract. Keep one
logical registry and one authoritative `spec.config`; cached views cannot independently change it.
Registration, names, retirement and other cross-authority changes retain checked coordination.
Storage engines, automatic migration/failover and exact service placement are not selected here.

**Change/recovery example:** a search Resource starts at limit 10/R7. Its control applies 15 and
the responsible service retains 15/R8 with the result, but the response to management is lost.
The Interface recovers the result from that service; an aggregate view still showing 10 identifies
its age. If the provider effect happened before any confirmation was retained, the outcome is
uncertain and still requires effect inspection or supported deduplication, never blind replay.

**Remaining ruling — outage admission:** distinguish keeping supervised processes alive, retaining
results of already-dispatched work, and admitting new effects. A live Agent process or graph is
not blanket permission for every later tool call or node. A management/observation outage also
need not imply that every authorization source or instance endpoint is unavailable.
The user requested [concrete outage examples](../../research/CLOUD_AUTHORITY.md#concrete-outage-comparison-before-ruling)
before deciding. Those examples distinguish frontend, registration, policy, provider and executor
failure; the permission-outage ruling remains open. If registration and authorization share one
process initially, that process's outage can block new capability calls even while executors
remain alive and retain results. A separate logical responsibility is not a deployment guarantee.

Recommended v0 baseline, **not yet accepted**: retain supervision and recoverable results; allow
already-dispatched operations to finish within their admitted contract; hold new affected effects
when current authority cannot be established. A still-reachable authority can admit permitted
work. Existing explicit stop/revocation rules remain applicable. This proposes no offline grants
and does not silently cancel a database migration or invent a replacement Agent.
Outcome retention does not itself authorize delivery to a caller; managed reads/delivery still
check current access. Holding dispatch need not mean indefinite queuing: pending versus temporary
unavailability follows the capability contract. No automatic replay policy is chosen here.

The alternative allows explicitly bounded offline grants, with validity, expiry and delayed
withdrawal semantics. That can improve availability but requires a further permission contract.
Current native-versus-isolated enforcement limits remain visible under either choice; Conduit
cannot claim to stop unmediated native tool use merely by blocking a managed request.

**Example:** if management disconnects after a migration was dispatched, its executor can retain
completion evidence. A subsequent protected migration or tool request waits if it cannot establish
current authority. Unrelated admitted work remains separately supervised. Whether that baseline
is sufficient or offline grants belong in v0 is the next collaborative ruling.

**Proposed evidence:** experiment 002 uses two execution processes on one machine, separately
faulting management, worker, capability, policy and observation links. Confirmed configuration
and outcomes commit at the instance authority; aggregate views recover without overwriting newer
state. Test retained supervision, held unauthorized/uncertain effects and verified reattachment.
Production remote support, specific crash-survival guarantees and when a two-machine proof is
required remain open. The protocol does not promise those results before testing.

I3 worker details and I4 persistence choices follow these accepted authority boundaries. Frontend
detachment, explicit resume of stopped Agent work, current authority checks and no duplicate
unknown work remain accepted. This review does not reopen those behaviors.

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

**Accepted:** public plugin invocation uses MCP with a small versioned Conduit profile for
instance/operation context and lifecycle/recovery semantics. Published contracts may bind authored
code, existing local/remote MCP services through compatible adapters, or both. No new executable
is required for a declarative contribution. Independently usable MCP surfaces preserve managed
ownership/access/evidence without forcing every internal export or object to become a standalone
server. Hosted implementations cannot inherit a binary-pinning guarantee; retain exact
declarations/adapters and report actual compatibility evidence. Profile details remain open.

**Recommendation for code-backed plugins:** use exact-version executable workers for externally
supplied implementation code. Standard MCP tools/resources and optional features carry compatible
exchanges. Internal trusted Rust traits
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

**Rulings still needed:** execution/distribution boundaries within the accepted custody and
authority split, with outage requirements still pending. MCP and implementation-binding modes are settled; exact artifact pinning and replacement
on upgrade are already accepted for Conduit-controlled code. The experiment must test whether
the profile stays small before treating its method set as stable. Standard protocol support alone
does not establish provider lifecycle, effect recovery or partition behavior.

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

**Ruling needed:** choose local persistence for the accepted authorities, accounting for the remaining outage requirements.
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

**Deferred to the end of review:** discuss syntax across all contracts and templates together
after the conceptual questions are settled. Publication with implementations is already accepted;
the field layouts, identifiers and schema-authoring choices below remain proposals.

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
