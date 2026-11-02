# Conceptual closure review

Updated: 2026-11-02. **Current work: close the conceptual model together.** Concrete package,
plugin-reference, graph-binding and schema syntax is deferred to a future PR. The latest history
package example is preserved as a proposal, not adopted syntax. No implementation has started.

The five-family model and the earlier R1–R7 / I1–I6 rulings remain accepted. I7 also settled
published contract authority and composition without a mandatory primary plugin. This audit
separates remaining semantic seams from previously settled decisions, drafting work and future
features. It does not guarantee that implementation will reveal no further counterexamples.

## Next discussion

**C1 and C2 are complete; C3 remains.** Accepted outcomes are marked; the C3 recommendation is
not a ruling. Work through its concrete scenario, then record the accepted result in the relevant
design file. Do not restart the entire decision history or turn each schema field into a new question.

### C1 — What does conformance promise?

**Already settled:** contracts are language-neutral and authoritative; templates, finite graphs,
operations, requirements and instance behavior have public semantics. MCP is the plugin invocation
base, with a Conduit profile. Components can expose supported capabilities independently. A concrete
frontend, Rust crate layout or storage engine does not define those public semantics.

**Accepted:** conformance describes explicitly supported capabilities. Clients/plugins can support
limited surfaces; a runtime claiming portable template execution must honor public graph, control,
lifecycle and recovery semantics. Required executable behavior cannot silently become opaque.
The [canonical contract](design/CONFIGURATION.md#declared-capability-conformance) records this ruling.
Exact names and syntax remain deferred.

**Accepted evidence threshold:** the first conformance milestone requires an independently authored
client and plugin working with the reference runtime through published contracts and conformance
tests. A second complete runtime is deferred. This tests the supported extension surfaces without
claiming runtime-to-runtime portability. C1 is complete; exact profiles, syntax and fixtures remain
for the future authoring PR.

**Why now:** collaborator PR #2 makes this choice visible. Its opaque setup/runtime proposal would
lose the portability we have prioritized. [Comparison and sources](proposals/COLLABORATOR_REVIEW.md).

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
or a universal serialized identity chain. The [dependent-call contract](design/CONDUITS.md#authority-across-dependent-calls)
records the boundary. C2 is complete; concrete propagation and checking mechanisms remain authoring work.

### C3 — What does retirement guarantee across connected objects?

**Already settled:** setup completion, runtime readiness and operation outcomes differ. Retry reuses
valid progress; uncertain effects are not blindly replayed. Agent deletion requires confirmed
execution end. Pool deletion requires explicit Resource retirement. A Conduit ends after its last
logical consumer/peer detaches; external effects may outlive that detach. Backing data is preserved
unless explicitly cleaned up. Incoming references alone do not cause cascading deletion.

**Still to decide:** the minimum common stop/retire/delete guarantee for the remaining families and
how it coordinates endpoint activity, child setup work and live dependent connections. Which duties
must be confirmed before unregistration, and which can remain recorded as incomplete or uncertain?
Provider-specific cancellation and cleanup still belong to their contracts.

**Example:** retire a Linear Resource while a tool write is in flight and two Agents remain attached.
Stopping its local bridge does not prove the upstream write was cancelled; neither Agent's own
execution or the shared authentication Resource should disappear automatically. An Interface whose
frontend closes is a different case from explicitly retiring the Interface itself.

**Recommendation:** use the existing Agent/Pool rules as the starting point: stop admitting affected
new work, account for owned activity and outstanding effects, and report incomplete retirement when
required confirmation is missing. Preserve dependent identities and unrelated/shared capabilities;
report their unavailable relationship. Define the minimum observable guarantees, while allowing
capability-specific cancellation/recovery and avoiding a single universal lifecycle enum.

## Review outcomes

Accepted details now live in the [design topic files](DESIGN.md#contract-map). Review documents
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

I1–I7 remain summarized in the [implementation review](proposals/implementation-v0/REVIEW.md).
The earlier history remains in Git; removing repeated ruling prose does not withdraw accepted rules.

## Design-fit audit: code-optional plugins and independent MCP access

A plugin may bind existing MCP services without new code; it cannot manufacture unsupported
filtering, recovery or lifecycle guarantees. Independently exposed managed capabilities retain their
current access checks, Pool ownership where applicable, fixed implementations and operation evidence.
An ordinary MCP implementation used outside Conduit supplies only its own declared guarantees.
No process per plugin, mandatory combined gateway or Interface-only invocation route is required.
C1 asks how those different compatibility claims should be named and tested, not whether to allow them.

## Initial experiment

[001](../experiments/001-shared-access/README.md) is a static workflow packet;
[002](../experiments/002-runtime-plugins/README.md) is the accepted synthetic runtime/composition proof.
Neither has executed runtime/provider tests. Keep their existing evidence and scope. After C1–C3,
translate the accepted clarifications into conformance cases before finalizing syntax. Do not add
Sealmesh, VMs, a root model, remote deployment or a second full runtime to the experiment silently.
Native two-Claude/two-Codex account validation remains a separate required goal.

## Drafting and deferred work

| Work | Treatment |
| --- | --- |
| Package maps/aliases, export references, file layout, schema dialect, API identifiers and exact messages | Park in the [authoring proposal](proposals/authoring-syntax/README.md); review together in a later PR after conceptual closure. |
| Crate organization, worker grouping, queue sizes, retry bounds, storage settings and serialization | Engineering/conformance work within accepted contracts. Bring back only a concrete semantic conflict. |
| Binding/default name scope, compatibility matching, output/result mapping and typed secret handles | Specify in the authoring pass; tests must preserve fixed bindings, current authority and declared recovery. Syntax cannot decide these accidentally. |
| Full remote/multi-user support, automatic placement/failover, stronger sandbox drivers | Future extensions with separate deployment evidence. |
| Root Agent, Sealmesh adoption and VM-per-Agent default | Collaborator proposals to evaluate; not prerequisites or accepted changes. |
| Automatic wake, general authored loops, in-place definition upgrades, marketplace, advanced publication | Remain deferred. |

## Closing this pass

Resolve C1–C3 collaboratively, reconcile the collaborator proposals against those results, and
check that each accepted rule has one canonical home and a concrete acceptance example. This closes
the current conceptual review, not all possible future design work. The next PR can then select
syntax and build conformance fixtures from the contracts. Existing examples are inputs to that
work, never a reason to declare whatever they happen to contain mandatory.
