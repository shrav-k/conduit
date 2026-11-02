# Implementation review status

Updated: 2026-11-02. I1–I6 and I7's contract-authority/composition rulings are accepted.
The [conceptual closure review](../../REVIEW.md) is complete, including C1–C3. Specific
syntax is parked for a later PR; implementation has not started. This summary preserves the
I1–I7 references without repeating a parallel detailed specification.

## Review priority: plugin and template authors

The durable product is the public protocol and authoring structure. Prove that an author can
compose existing contributions, add a small operation, and supply a managed capability without
changing core dispatch. Start with lightweight support and real authored plugins; grow helpers
from demonstrated needs. Rulings remain collaborative. A recommendation or example is not accepted
merely because it is detailed or statically parseable.

## I1 — Independent execution and management

**Accepted:** separate management and execution services; shared registration/policy authority;
one designated authority for each instance's definition, confirmed settings and outcomes; backend
ownership of domain data; source-aware Interface views. Services may host many instances.
No offline grants: supervision and dispatched results survive where supported, while new managed
requests wait if current permission cannot be verified.

The first proof uses management and two executor processes on one machine with controlled failures.
Two-machine validation precedes remote-support claims. See the
[authority contract](../../design/MODEL.md#shared-policy-and-instance-owned-state) and
[outage contract](../../design/MODEL.md#v0-outage-admission).

## I2 — First real Agent path and supported platform

**Accepted:** after the synthetic proof, native Claude/Codex adapters precede Conduit's own Rust
model/tool loop. Native subscription-account validation is macOS-first, with a portable core and
OS adapters. The two-Claude/two-Codex account goal remains; synthetic identities do not satisfy it.
Pi is research/frontend inspiration, not the runtime dependency. See
[Agent runtime](../../design/AGENT.md#runtime-and-presentation).

## I3 — Plugin execution and distribution

**Accepted:** published contracts and implementation bindings may use code, existing MCP services,
graphs or combinations. MCP plus a Conduit profile is the public invocation boundary. Independent
managed surfaces retain authority/evidence obligations. Endpoint instances select/manage their
plugins; Conduits compose requirements and coordinate compatible behavior.

Requirements compose obligations; graphs compose typed operations. Composite public results derive
from declared public bindings by default, with explicit domain construction available. Reuse graph
recovery and valid child progress; preserve effects and uncertainty. Requirement contracts expose
fulfillment duties; optional conflict checks do not replace individual enforcement.

Authored code normally uses supervised instance-associated helpers; trusted built-ins may be
in-process. There is no process-per-plugin rule. Bundle first-party plugins and allow explicit local
installation; defer marketplace and automatic cross-language dependency resolution. Experiment 002
is scoped to prove executable and requirement composition together. See
[extension model](../../design/MODEL.md#authoring-concepts-beneath-the-object-families),
[plugin boundaries](../../design/MODEL.md#plugins-and-independently-accessible-mcp-components) and
[requirements](../../design/CONDUITS.md#composable-relationship-requirements).

## I4 — Local transactional authority

**Accepted:** policy is independent of Interfaces; recorded participation grants govern managed
access; issued grants do not depend on issuer ancestry. Policy applies composable requirements
with explicit targets and live ownership-based matching. Bounded issuance can supply mandatory
use restrictions. The system Pool owns separate management and access-management Resources.

The locally loadable root Interface directs first setup and explicit recovery, with verified
local-owner authority for those foundational operations. Ordinary access uses managed grants;
failed management does not reopen initialization or grant offline access.

Publish a minimal persistence plugin contract, loaded as an authority-service runtime dependency
in v0. Bundle SQLite with separate local service databases. A future Pool-owned store remains
possible; there is no distributed transaction or atomicity with external effects. See
[foundational system](../../design/MODEL.md#interfaces-and-the-user-workflow),
[access](../../design/CONDUITS.md#authority-and-current-access), and
[persistence](../../design/MODEL.md#plugins-and-independently-accessible-mcp-components).
[AUTHORIZATION](AUTHORIZATION.md) retains detailed examples/proposed representations.

## I5 — Credential custody and account compatibility

**Accepted:** default Conduit-retained secrets to a replaceable OS-store implementation bundle,
initially macOS Keychain. Config/evidence hold references, not secrets; no silent plaintext fallback.
Interface-guided adapter enrollment/unlock reuses existing access and surfaces required human
interaction. Native Agent login and external secret ownership remain distinct. See
[shared authentication and custody](../../design/POOLS.md#shared-access-and-authentication).

## I6 — What observability captures by default

**Accepted:** extensible observation plugins/Conduits, lightweight correlated traces/diagnostics,
small health metrics and optional OpenTelemetry export through Interface inspection. Synthetic tests
record all instrumented traces at source with bounded fixture capture and visible gaps. Real full
content needs explicit recording policy; credentials stay out of ordinary logs. Authoritative
operation/recovery evidence is unsampled and independent of optional exporters. See
[observability](../../design/MODEL.md#interface-observability).

## I7 — Authoring conventions and multi-plugin graphs

**Accepted:** published language-neutral contracts govern public APIs and behavior. Contracts may
be handwritten or generated, then reviewed/versioned; implementations conform. Complete templates
can bind compatible contributions from multiple plugins without a mandatory primary. Standard
templates and declared defaults resolve to explicit retained bindings, without silently merging
installed packages or falling back from an invalid explicit choice. See
[composition](../../design/MODEL.md#authoring-concepts-beneath-the-object-families).

**Deferred:** concrete package, template and contract syntax. The named plugin map, uniform export
references, package-local/default scope, schema dialect and API identifiers have not been ruled on.
The [authoring packet](../authoring-syntax/README.md) preserves the examples for a later PR.

## After review

C1 conformance, C2 dependent-call authority and C3 retirement are settled in
[REVIEW](../../REVIEW.md). Concrete authoring syntax returns in a later PR.
Then derive conformance cases and review the authoring packet as one connected contract. Runtime
mechanisms and milestones in [ORGANIZATION](ORGANIZATION.md) remain implementation proposals within
accepted boundaries. Do not begin coding or silently reopen settled choices as part of documentation
cleanup. A concrete counterexample may require a new ruling; routine field names/tuning do not each
need their own conceptual review.
