# Conceptual closure review

Historical closure record. C1–C3 remain accepted in the [active protocol reference](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/README.md).
Use [current review](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/REVIEW.md) for outstanding work; this is not an active ruling queue.

Updated: 2026-11-02. **The current conceptual review is complete: C1–C3 are accepted.** Concrete package,
plugin-reference, graph-binding and schema syntax is deferred to a future PR. The latest history
package example is preserved as a proposal, not adopted syntax. No implementation has started.

The five-family model and the earlier R1–R7 / I1–I6 rulings remain accepted. I7 also settled
published contract authority and composition without a mandatory primary plugin. This audit
separates remaining semantic seams from previously settled decisions, drafting work and future
features. It does not guarantee that implementation will reveal no further counterexamples.

## Next discussion

There are no outstanding rulings in this conceptual review. The current
[repository and distribution review](REPOSITORY-2026-11-02.md) considers organizational boundaries
before a separate authoring/conformance PR: complete contract and template examples, schemas and
wire/profile bindings, followed by implementation. Existing examples remain proposals until reviewed together.
Concrete counterexamples can reopen a semantic issue; this is not a claim that every future extension
or engineering detail is settled.

### C1 — What does conformance promise?

**Already settled:** contracts are language-neutral and authoritative; templates, finite graphs,
operations, requirements and instance behavior have public semantics. MCP is the plugin invocation
base, with a Conduit profile. Components can expose supported capabilities independently. A concrete
frontend, Rust crate layout or storage engine does not define those public semantics.

**Accepted:** conformance describes explicitly supported capabilities. Clients/plugins can support
limited surfaces; a runtime claiming portable template execution must honor public graph, control,
lifecycle and recovery semantics. Required executable behavior cannot silently become opaque.
The [canonical contract](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/CONFIGURATION.md#declared-capability-conformance) records this ruling.
Exact names and syntax remain deferred.

**Accepted evidence threshold:** the first conformance milestone requires an independently authored
client and plugin working with the reference runtime through published contracts and conformance
tests. A second complete runtime is deferred. This tests the supported extension surfaces without
claiming runtime-to-runtime portability. C1 is complete; exact profiles, syntax and fixtures remain
for the future authoring PR.

**Why now:** collaborator PR #2 makes this choice visible. Its opaque setup/runtime proposal would
lose the portability we have prioritized. [Comparison and sources](../collaborator-proposals/REVIEW.md).

### C2 — Whose authority governs a dependent call?

**Already settled:** every managed caller needs recorded participation authority; requirements
narrow access; shared credentials are not caller grants. Resource-to-Resource calls use Conduits.
Requirements have declared applicability and enforcement duties. Reads and delivery recheck current
access, and derived knowledge retains declared source-access dependencies.

**Accepted:** a Resource serving an Agent uses its own authorized dependency connections while
preserving the authenticated originating caller and enforcing applicable restrictions on requested
effects and returned content. Autonomous work uses the Resource's own authority. Capability contracts
define the enforcement; callers do not need direct grants to every internal backing service.

For example, search may index Euphony and Demo under its own authority, while an Euphony-only Agent
cannot retrieve Demo through a query. A shared service credential or dependency grant cannot widen
that caller's permitted effects/results. This does not introduce issuer ancestry, token forwarding
or a universal serialized identity chain. The [dependent-call contract](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/CONDUITS.md#authority-across-dependent-calls)
records the boundary. C2 is complete; concrete propagation and checking mechanisms remain authoring work.

### C3 — What does retirement guarantee across connected objects?

**Already settled:** setup completion, runtime readiness and operation outcomes differ. Retry reuses
valid progress; uncertain effects are not blindly replayed. Agent deletion requires confirmed
execution end. Pool deletion requires explicit Resource retirement. A Conduit ends after its last
logical consumer/peer detaches; external effects may outlive that detach. Backing data is preserved
unless explicitly cleaned up. Incoming references alone do not cause cascading deletion.

**Accepted:** retirement stops new work, releases the instance's own activity/attachments, and permits
unregistration only when its contract's required shutdown duties are confirmed. Missing confirmation
leaves retirement incomplete. Preserve shared dependencies, other instance identities and operation
evidence. Already-dispatched external effects may outlive local shutdown; no cancellation or rollback
is inferred. Contracts define the duties without one universal lifecycle enum.

For example, retiring Linear access preserves the Agents and shared authentication Resource, while
an in-flight write keeps its recorded or uncertain outcome. Frontend detachment is distinct from
Interface retirement. The [canonical lifecycle rule](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/LIFECYCLE.md#retirement-and-unregistration)
applies alongside existing Agent/Pool checks and separate cleanup/purge contracts. C3 is complete.

## Review outcomes

Accepted details now live in the [design topic files](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/README.md#contract-map). Review documents
summarize decisions and link to their authority; they do not repeat a second specification.

| Area | Settled direction |
| --- | --- |
| Object model | Interface, Agent, Resource, Pool and Conduit; reusable templates and concrete instances. |
| Extensibility | Published contracts, code/MCP/graph-backed plugins, composable requirements, finite graphs; no mandatory primary plugin. |
| Configuration | Strict typed YAML, stable IDs, explicit creation/updates, fixed instance definitions and exact implementations, named controls for supported changes. |
| Relationships | Endpoint-managed implementations, explicit open/join, per-consumer participation, narrowing complete access paths, new identity after a flow ends. |
| Access | Independent governing requirements, recorded participation grants, current checks, no offline grants or issuer-dependent cascades. |
| Resources and Pools | One Pool owner per Resource; explicit available-template entries and provision/use-existing intent; shared service authentication. |
| Authority and recovery | Shared registration/policy; one authority per instance; root Interface setup/repair; explicit resume and honest uncertain outcomes. |
| Reference implementation | Separate management/executors, supervised instance-associated helpers, SQLite service dependencies, replaceable OS credential custody. |
| First proof | One machine, management plus two executors; composition/requirements, controlled failures and Interface evidence; native macOS Agent adapters next. |

I1–I7 remain summarized in the [implementation review](IMPLEMENTATION.md).
The earlier history remains in Git; removing repeated ruling prose does not withdraw accepted rules.

## Design-fit audit: code-optional plugins and independent MCP access

A plugin may bind existing MCP services without new code; it cannot manufacture unsupported
filtering, recovery or lifecycle guarantees. Independently exposed managed capabilities retain their
current access checks, Pool ownership where applicable, fixed implementations and operation evidence.
An ordinary MCP implementation used outside Conduit supplies only its own declared guarantees.
No process per plugin, mandatory combined gateway or Interface-only invocation route is required.
C1 settles declared-capability conformance and the independent-client/plugin proof; exact profile
names and fixtures follow in the future authoring PR.

## Initial experiment

[001](https://github.com/shrav-k/conduit/blob/de6566ad8a2ea408876a66ab04930734c7b334de/experiments/001-shared-access/README.md) is a static workflow packet;
[002](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/experiments/002-runtime-plugins/README.md) is the accepted synthetic runtime/composition proof.
Neither has executed runtime/provider tests. Keep their existing evidence and scope; translate the
accepted C1–C3 clarifications into conformance cases in the later authoring PR before finalizing syntax. Do not add
Sealmesh, VMs, a root model, remote deployment or a second full runtime to the experiment silently.
Native two-Claude/two-Codex account validation remains a separate required goal.

## Drafting and deferred work

| Work | Treatment |
| --- | --- |
| Package maps/aliases, export references, file layout, schema dialect, API identifiers and exact messages | Park in the [authoring proposal](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/proposals/authoring-syntax/README.md); review together in a later authoring/conformance PR. |
| Crate organization, worker grouping, queue sizes, retry bounds, storage settings and serialization | Engineering/conformance work within accepted contracts. Bring back only a concrete semantic conflict. |
| Binding/default name scope, compatibility matching, output/result mapping and typed secret handles | Specify in the authoring pass; tests must preserve fixed bindings, current authority and declared recovery. Syntax cannot decide these accidentally. |
| Full remote/multi-user support, automatic placement/failover, stronger sandbox drivers | Future extensions with separate deployment evidence. |
| Root Agent, Sealmesh adoption and VM-per-Agent default | Collaborator proposals to evaluate; not prerequisites or accepted changes. |
| Automatic wake, general authored loops, in-place definition upgrades, marketplace, advanced publication | Remain deferred. |

## Closing this pass

C1–C3 were resolved collaboratively and incorporated into their canonical design topics. The
collaborator comparison identifies adopted refinements, adapted suggestions and preserved dissent;
original contributions are archived with attribution. No conceptual ruling remains in this queue.
The next PR can select syntax and build conformance fixtures from the contracts. Existing examples
are inputs to that work, never a reason to declare whatever they happen to contain mandatory.
