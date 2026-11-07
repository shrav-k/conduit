# Round 6: simplicity and usefulness of Conduit

Codex, reader 1. Written 2026-11-07 against `main` at `c4d7d33`, after pull request 7.
This expands [the findings report](reader-1.md) at the owner's explicit request for a broader,
thorough critique to take back to Claude. It is a report, not an accepted redesign.

Only [the specification](../../../../spec/README.md) binds. The criteria are the owner's
[purpose and preservation tests](../../../AUDIT.md#purpose), the provider direction of rulings
40–43, and the existing experiment program. No specification, historical record, engine or
provider is changed by this report. Recommendations that change behavior need a logged
decision. A proposed test below is not an executed test or an additional readiness gate.

## Judgment

**Conduit's useful center is coherent: establish a shared thing once, give each participant
explicit access, inspect and revoke that access, and preserve uncertainty about effects. Its
first-user simplicity and practical usefulness are not yet demonstrated.** The concern is not
that five families or 452 promises are inherently too many. It is that several mechanisms make
an author learn infrastructure, contract packaging and recovery before obtaining the central
benefit, while some of the most attractive benefits require a stronger client or implementation
than ordinary MCP compatibility establishes.

The strongest criticisms are these:

1. **Restricted sharing is conditional on integration work.** A plain MCP server can be exposed
   under operation allowlists, but cannot apply Conduit filters. The flagship example of two
   agents sharing a login with different argument or returned-content limits needs a suitable
   Conduit-aware implementation. This cost is part of the product, even when it lives outside
   the engine.
2. **Narrow delegation is narrower in some dimensions only.** Bounds do not restrict filters,
   and management registration cannot be limited to an approved template. The language can
   delegate broad administration; it cannot directly express every natural least-authority
   management task suggested by “agents can operate the system.”
3. **Retry protection depends on the caller preserving identity.** An ordinary repeated tool
   call without Conduit's idempotency key is new work. Engine correctness alone cannot make a
   client's lost-response recovery safe.
4. **The change cycle is harder than the initial declaration.** Frozen instances are valuable,
   but secret rotation, provider replacement, editing local contracts, repairing nested
   instances and reconstructing access impose separate burdens. A first-use demonstration
   that ends after setup would miss much of the cost.
5. **There are credible semantic simplifications.** Remove a layer of implementation defaults;
   reconsider control constraints that discovery does not show; reconsider replacement's frozen
   local project context; remove the special prohibition on stable IDs in actor requests if
   its benefits do not outweigh its costs. These are decisions, not wording fixes.
6. **More prose review cannot settle these tradeoffs.** Resolve the safety contradictions in
   findings 6-1-01 and 6-1-02, then use the already planned S1 tracer, S2 restricted sharing,
   recovery scenarios and provider swaps to measure the design. Another large feature rewrite
   before those measurements would risk exchanging known complexity for untested complexity.

## How to read the recommendations

`D01`–`D22` identify design critiques, not new numbered promises or severity findings. Each
distinguishes an existing rule from a judgment or proposed change. “Priority” below is an order
for consideration, not a new project schedule. Read the separate six findings for exact defect
quotes, counterexamples, counterarguments and smallest repairs.

| Priority | Items | Recommended disposition |
| --- | --- | --- |
| Resolve before relying on safety | Findings 6-1-01, 6-1-02; 6-1-03 and 6-1-04 for deterministic implementation | Decide the incompatible rules and write failing cases. Do not treat a severity-label change as a safety result. |
| Test the central product claim first | D01–D05, D08–D10 | Qualify clients and implementations; exercise restricted sharing, narrow administration and the change/recovery cycle. |
| Put concrete simplifications to the coordinator/owner | D06, D07, D11–D14 | Compare the stated alternatives. Each changes behavior or the interface; none is applied here. |
| Improve the next implementation pass | D15–D22 | Measure integration and retention costs, expose current semantics clearly, and strengthen cross-rule evidence using the accepted program. |

## What should be preserved

These are substantial strengths. Removing them would need evidence of a simpler model that
keeps the same properties, rather than a shorter example alone.

| Design choice | Why it earns its place | Relevant specification |
| --- | --- | --- |
| A Conduit is a separately identified object | Access can be inspected and closed without destroying the consumer or the shared Resource. An allowlist is explicit; offering a capability does not grant it. | CON-001, ACC-001, ACC-003, ACC-011 |
| The five families distinguish behavior | Actors may initiate; Resources act while serving; Pools own shared things; Conduits carry access. Merging nouns would move these distinctions into flags and exceptions. | OBJ-013, POOL-001, ACC-028, IFC-021 |
| Ownership does not itself grant access | A shared thing is held once; ownership and permission answer different questions. | POOL-001, ACC-028 |
| Resolved relationships use immutable IDs | Reusing a readable name does not silently redirect an existing relationship. | OBJ-009, OBJ-011, INS-020 |
| Registration, setup and effects are explicit | Reading a configuration is not permission to run models or repeat uncertain work. | INS-016, TPL-012, IMP-001, OPS-012, OPS-018 |
| Unknown is a first-class outcome | A lost response cannot truthfully be collapsed into “nothing happened.” | OPS-003, OPS-007, OPS-012, OPS-014 |
| Revisions protect controlled changes | Stale knowledge is rejected rather than silently overwriting a newer setting. | REV-001, REV-002, REV-007 |
| Provider mechanics are outside the relationship language | Users can choose backing systems without making the language a scheduler, secret store or process manager. This is a sound boundary, subject to actually proving the adapters. | PRV-001–PRV-005, SEC-020 |
| Unsupported filters are rejected | Silently dropping a restriction would make concise configuration dangerously misleading. | CON-022, ACC-015, IFC-019 |
| No general expression or workflow language | Literal bindings and ordinary implementations avoid turning configuration into another programming language. | OBJ-019, INS-012, TPL-017, TPL-018 |

Keep one capability per Conduit for now. Combining capabilities would save some documents,
but introduce questions about independent revocation and which restrictions cover which
operations. Keep one run per instance for now too (TPL-019); extra cooperating processes belong
behind a provider or ordinary implementation until an actual scenario demonstrates otherwise.
Neither count reduction is automatically a simplicity improvement.

## The first useful journey

The following is an analytical walk-through, not a runnable example. It makes explicit what
the accepted S1/S2 scenarios must establish. The existing project is a schema fixture with
invented digests and an absent example harness; it is not evidence that this workflow runs.

| User intent | What the current model requires | What should be measured |
| --- | --- | --- |
| Share one project with two agents | Suitable contracts/implementations and templates, installed dependencies, a Pool and Resource, Agent instances, Conduits, explicit setup | Which files the end user edits versus what reusable packages supply; time and failed attempts to the first successful request |
| Give the second agent less access | Separate operation allowlists, or a filter the selected implementation supports | Whether the desired restriction needs new code; whether inspection explains its scope and enforcer |
| Share one service login without giving it to either agent | A secret Resource, the shared service Resource's binding and Conduit to it, root authorization where SEC-008 requires it | That the service receives the shared credential and neither Agent's run receives it; the actual trust boundary |
| Let an agent create an approved peer | Broad management access, or a trusted implementation exposing a narrower operation | Whether broad trust is acceptable; cost of the broker and of explaining its authority |
| Recover after a lost response | Stable request key, inspection of outcome, and sometimes reconciliation based on external evidence | Whether the client preserves the key; human steps needed; whether a new effect is accidentally requested |
| Change a setting or the implementation | Control with current revision, or template change plus explicit replacement of affected instances | Distinguishing a future-template change from a live change; changed IDs, downtime and access reconstruction |
| Rotate a credential or swap a runtime | Later connection/run or replacement, under the current lifetime rules | Which instances, bindings and Conduits must change; whether “swap” preserves the intended user workflow |
| Revoke and clean up | Close the specific Conduit, retire appropriate instances, inspect uncertain shutdown | That the other agent still works, new dispatch is refused, and surviving remote work is not misrepresented as stopped |

The shared Resource should remain one resource throughout ordinary sharing. Do not make the
demonstration “simpler” by giving each agent its own credential or private duplicate. That would
avoid the problem Conduit exists to solve.

## D01 — Make the useful subset of MCP compatibility explicit

**Evidence.** PKG-020 says a plain implementation “states no `supports`”; CON-022 refuses a
Conduit whose target cannot apply its filter. IFC-007 sends a plain server none of Conduit's
keys. IFC-045 declares no capabilities for implementation callbacks such as elicitation or
sampling. These are intentional boundaries, not newly discovered defects.

**Critique.** “Package an existing MCP server” and “share its login under different limits”
are not interchangeable promises. Existing operation names and input schemas may be enough
for an allowlist. They are not enough to enforce a per-caller repository path, spending limit
or filtered search result. Changing `plain` to false is not an adapter: the implementation must
actually honor the extension and the restriction.

**Recommendation.** Make a qualification table part of S2/S7. Include an unchanged plain server,
a small adapter and a Conduit-aware implementation. State exactly what each supports, count
the adapter work, and use the same user task. Supply reusable implementations for the initial
restriction patterns that the product relies on. Do not introduce a generic proxy that claims
to understand arbitrary tool semantics.

**Cost and decision.** Reusable adapters are work outside the language but inside the adoption
cost. If each ordinary integration needs bespoke security code, Conduit may be more useful as
a framework for integration authors than as a simple configuration language for the first
developer. That is a hypothesis to test, not a conclusion established by the current files.

## D02 — Treat retry-capable clients as a separate qualification

**Evidence.** IFC-023 makes the idempotency key optional custom request metadata. OPS-011 says
a request without a key “is a new invocation,” even with identical contents. Plain targets
never count as deduplicating under PKG-020/OPS-012.

**Scenario.** A client loses the response to “create ticket,” then its model calls the same
tool with the same arguments. Without the original key, the second call is a second request
for work. The engine may be perfectly conforming and still create two tickets. This is not
the protocol-classification defect in finding 6-1-01; it is the intended meaning of unkeyed
requests.

**Recommendation.** Test at least four distinct cases: a key-preserving client and a client
without keys, each against a plain and a deduplicating implementation. Report what happens on
success with lost response, unknown outcome, and an intentional second identical action. A
client integration that promises reliable retries must preserve keys across the retries it
initiates and distinguish them from new user work. Do not deduplicate by argument equality:
two identical payments or messages can be intentional.

The product's explanation should distinguish “Conduit does not implicitly repeat an effect”
from “this client will never ask for the effect twice.” The former is the language's promise;
the latter requires evidence about the client.

Qualification has independent consumer and implementation dimensions; `plain` alone does not
describe the whole connection:

| Intended benefit | What must be demonstrated |
| --- | --- |
| Basic operation allowlist and attribution | Authenticated consumer connection and the engine's Conduit decision; a plain target is sufficient |
| Argument/effect/returned-content restriction | Target implementation supports and correctly applies the particular Requirement contract |
| Reuse of a successful keyed result | Consumer preserves the original key and contents; current access and replay rules permit delivery |
| Retry of an unknown effect without duplication | Consumer preserves the key; target satisfies the specified deduplication contract; a plain target does not qualify |
| Model sees a refusal or failure | Real client exposes the relevant protocol error or result content, not merely receives it |
| Correct revocation experience | Next dispatch is refused even with cached tools; client refresh and error handling are observed separately |
| Cancellation and long-running work | Client/target combination supports the selected task behavior and distinguishes cancellation from proof of no effect |

## D03 — Delegation needs an honest description of its dimensions

**Evidence.** ACC-029 bounds Pools, targets and operations. ACC-030 explicitly says a bound
“never by filters” and trusts its consumer with unfiltered access within it. Naming a Pool
covers Resources created there later. Closing the grantor's access does not close Conduits
already opened for others.

**Scenario.** An agent reads only `/project/public` through a filter but also has authority to
open access to the directory's read operation. That opening authority can permit an unfiltered
Conduit. The filtered read grant never constrained the separate administrative authority.
Likewise, revoking a delegator does not automatically revoke an independently opened grant.
Both are specified behavior, not an authorization bypass.

**Recommendation.** Show effective authority in two parts: what the agent may do now, and
what access it may create, including future Pool members and the absence of filter bounds.
Use a concrete negative example beside every “delegation only narrows” explanation. Ask the
owner whether the S2 workflow needs filter-preserving delegation. If it does, choose an explicit
restricted broker or a focused future bound design; do not quietly restore the removed policy
and requirement-composition systems.

**Tradeoff.** The current design avoids defining implication between arbitrary filters. Keep
that simplification unless restricted delegation is important enough to justify a precisely
limited additional model. Independent grants should not acquire hidden revocation ancestry
merely to make the wording sound stronger.

## D04 — Narrow peer provisioning is not directly expressible

**Evidence.** ACC-030 says `register.v1`/`replace.v1` on management trusts the consumer for any
object authorized against management alone. ACC-033 gives the routing; CON-022 and the engine's
manifest provide no custom filters on management operations. A Conduit's `bound.v1` is for
`access`, not management (CON-028).

**Scenario.** “Let the coding agent start a reviewer from this approved template, but register
no other Agent or template.” The engine's management grant cannot express that restriction.
A Pool's additional registration permission narrows some Resource registration, but does not
solve the Agent-template case.

**Options.** Keep broad administration as an explicitly trusted operation; expose a trusted
Resource operation whose own contract/configuration permits only the approved template; or
design a focused registration constraint. Recommend testing the broker option in S1 first,
while recording its trusted code and setup cost. Its own operation must enforce the constraint:
a custom filter carried onward to management would fail because management does not support it.

This is a significant limit on the advertised management usefulness. It is also a legitimate
place to keep v0 small if the owner accepts broad trust. The report does not recommend a
generic new policy language merely because the current boundary is coarse.

## D05 — A filter contract must explain meaning, not just advertise support

**Evidence.** CTR-005 supplies Requirement inputs; CON-022 matches support by address and
export name; IFC-019 requires applying the filter to the call and what it returns. ACC-038
usefully identifies the implementation applying it. Findings 6-1-02 and the S8 scenario expose
why execution and output semantics matter.

**Critique.** Two implementations saying they support `rate-limit.v1` need not be predictably
substitutable to an author unless the contract says whose requests are counted, the time window,
whether failed attempts count, whether a repeat consumes quota, and where shared state lives.
A path filter must decide how indirect references and derived results are treated. These are
type/requirement semantics, not reasons to teach the engine every domain.

**Recommendation.** Make the first small set of useful requirements complete contracts with
shared vectors across two implementations. Include changed external authorization state,
cached output delivery, onward calls and revocation. Keep enforcement with the component that
understands the data; make its declared behavior testable. Prefer one well-defined restriction
that works across two real providers to many nominally portable requirement names.

Do not solve finding 6-1-02 by assuming all filters are timeless predicates. That would rule
out useful time-, quota- and state-dependent restrictions unless the owner deliberately chooses
that smaller language and records the cost.

## D06 — Remove one implementation-selection fallback

**Evidence.** TPL-003/004 select template override, then Type default, then Operation default
(CTR-002). The example `directory.list.v1` defaults to `listing`, but its Type chooses
`files-server`, allowing the template to satisfy the single-run rule TPL-019.

**Critique.** A contract that describes callable behavior also chooses executable packaging.
Predicting what runs can require three separate locations. Adding an offered operation without
a suitable Type override can select a second run and make a new template invalid. Pinned existing
instances do not change; the surprise arises during new resolution or an explicit upgrade.

**Recommended simplification for comparison.** Remove Operation-level implementation defaults;
retain Type defaults and explicit template selections. This separates the reusable operation
contract from the default executable choice and deletes a semantic fallback.

**Cost.** Type authors must write mappings now inherited from operations, including core types.
Independent operation packages lose convenient automatic selection. This reopens historical
core-and-schemas choice C42, accepted by ruling 39; it is not an editorial change. Rewrite the
S1 package both ways, count repeated mappings, and ask a fresh reader to predict the selected
run. Keep the existing mechanism if its actual reuse benefit outweighs the extra selection
layer. A shorter template alone does not settle the comparison.

## D07 — Discovery should show the constraints the target will actually enforce

**Evidence.** TPL-010/011 allow a template to narrow control inputs. The example
`project-directory` limits `set-read-limit.v1` to 50, while its contract's `read-limit.v1`
allows 100. IFC-033 presents “the contract's own” schema. A template's controls map can also
exclude a control while its Type still offers the capability containing it (TPL-013).

**Scenario.** An agent follows the published tool schema and supplies 75, then receives
invalid for a fixed target constraint that discovery did not show. This is a discoverability
mismatch, not permission to bypass the narrower constraint. The same issue arises when a
presented capability contains a control permanently unavailable on this instance.

**Recommendation.** Prefer effective target input schemas and tool availability if template
narrowing stays. An alternative semantic subtraction is to remove per-template control input
narrowing and make specialized Type/Operation contracts express the restriction. The latter
loses convenient local limits and may multiply contracts; it is not automatically simpler.
Whichever is chosen, test the existing 50/100 example and an empty controls map using only the
tool description given to the agent. Schema publication must preserve reference semantics as
finding 6-1-04 recommends. This is concrete interface work, not a request for more explanatory
prose around an avoidable invalid call.

## D08 — Give the full change cycle the same attention as initial setup

**Evidence.** INS-020 keeps existing instances' resolved definitions after template changes.
LIF-009 explicitly registers a replacement for a nested instance and does not set it up.
Placement, ownership and the selected runtime are fixed for an instance's life (PLC-009,
POOL-001, PRV-006). CON-008 closes affected Conduits when their ends retire under its rules.

**Critique.** A file edit is not a desired-state rollout. An author has to decide whether a
change affects a future template, a live control, a new instance, a nested replacement or a
new access relationship. That complexity buys explicit effects and stable references, but it
is part of using the language, not an implementation detail.

**Recommendation.** Add a change-and-repair chapter to the S1 tracer: change a live setting,
upgrade a template, replace a nested Resource, then restore intended access. Record the exact
calls, identities and permissions involved. A frontend may show a pure preview of the required
explicit actions and their consequences; this is a tooling proposal compatible with the
direction, not an assertion that a new `plan` operation already exists. Let the user request
effects explicitly. Do not add automatic reconciliation, rollback or background model work to
hide the cost.

If the only usable upgrade path repeatedly recreates unrelated agents or relationships, that
is evidence for a targeted language change. Establish it with a complete trace first.

## D09 — Secret rotation is a useful stress test of immutability

**Evidence.** SEC-014 describes which later run sees a rotated value; SEC-013/023 cover
connection values. The specification's own secret-limit section explains that no operation ends a ready instance's
run and preserves that instance, so a running instance gets the new value through Conduit
alone in another instance made after retirement. A67 already records a possible restart verb.

**Scenario.** The developer rotates the shared service token. The running service still uses
the old value; changing the store was not a live configuration update. Retiring and replacing
it affects its ID and its access relationships. This is specified and honestly documented,
but it is a common maintenance task with a relatively large language-level consequence.

**Recommendation.** Exercise rotation in S2/S10, including access reconstruction and a failure
between retirement and replacement. Prefer a documented explicit replacement procedure first.
If it is disproportionately disruptive, bring the already-recorded restart candidate to the
owner with exact identity, in-flight invocation, hook, secret-read and stop semantics. A
convenient restart is not free: preserving identity while replacing execution creates new
failure cases. Do not turn this review into an unrequested accepted restart feature.

## D10 — Recovery must tell the actor what it can safely conclude

**Evidence.** OPS-014 lets an authorized actor record that an uncertain effect occurred or did
not, with what it checked. OPS-034 may report conflict after success because outputs are no
longer retained. ACC-033 routes reconciliation differently from the original target operation.

**Critique.** “Conflict” can mean an invocation is still running, a revision is stale, the
same key names different work, or the effect succeeded but its outputs cannot be replayed.
These are operationally different. Reconciliation is an assertion based on evidence, not an
engine discovery of external truth. A mistaken assertion can make a later retry inappropriate.

**Recommendation.** Preserve the small reason set, but make the already-required `ruleId`,
invocation identity and relevant status sufficient for a frontend to distinguish these cases.
Use S4/S5 to present what is known, what remains unknown, where the actor can verify it, and
which operations it is permitted to take next. Do not recommend “retry with a new key” simply
because cached outputs were lost. Do not let a model's unsupported guess become reconciliation
evidence. Contract-specific verification is useful; generic automatic effect detection is not
available merely because a status field exists.

Include partial setup, not just one external tool call: LIF-027 deliberately leaves already
opened Conduits and runs in place after a later part fails. OPS-022 may report unknown for
partial effects. The recovery transcript must identify those surviving relationships and runs;
it must not imply that a failed-looking setup restored the previous state.

This recommendation does not require the language to prescribe a UI. It asks the planned
inspection/vector work to carry enough portable facts for one to be built correctly.

## D11 — Reconsider the special local-context rule for template replacement

**Evidence.** SUB-009 resolves a replacing template's local contract and implementation
selections (`#name`) against its original submitted project. Nested `from` keeps the separate
template lookup rules of INS-025/NAM-022. A supplied manifest may contain only packages beside
`apiVersion`; new local contracts or implementations cannot replace that context. INS-020
separately preserves old instances.

**Scenario.** A developer edits a local Operation contract, its implementation item and its
template together. Replacing the template does not load the new local contract/implementation.
The author must unregister and re-register the relevant template/project context, or use a
published package workflow, while managing instance replacement separately.

**Proposal to compare.** Let replacement explicitly receive a fresh project context, resolve
it as registration does, and store the new template snapshot under the existing revision check.
Old instances keep old snapshots. This would remove the surprising special meaning of `#name`
on replacement while expanding the replacement request's inputs.

**Cost.** Submission digests, address collisions, reference lifetime and validation must be
reused precisely. This is not pure deletion, and it should not become an atomic multi-template
`apply` by accident. Recommend the current local edit cycle as an S1 test, then decide whether
its cost justifies this focused change. Frozen instances and a forever-frozen template context
solve different problems; retaining the former does not by itself justify the latter.

## D12 — Stable IDs should be usable for safe follow-up actions

**Evidence.** Inspection exposes resolved `{kind, id}` references (OBJ-009 and core reference
schemas). LIF-031 rejects that form in an actor's own request, while accepting it in an onward
request. Actor requests use names, which can be reused after removal. Controls have revisions,
but not every follow-up management action is a revision-checked control.

**Scenario.** An actor inspects Resource A, then A is removed and B takes its name. Acting on
that name refers to B. The actor cannot simply submit the exact object identity it inspected
using the already-defined reference form. This is an identity-selection usability problem,
not a demonstrated violation of the current name-resolution or authorization rules.

**Recommended simplification for review.** Consider allowing resolved IDs in actor requests,
with every existing kind, existence and authorization check intact. Keep readable names in
authored documents. An ID must confer no authority; a removed ID must not retarget to a reused
name. This deletes an actor/onward asymmetry and makes inspection results more reusable.

**Cost and precedent.** It reopens the reference-only choice D10/X2 of core-and-schemas,
accepted before ruling 39. The alternative is a separate expected-ID precondition on named
actions, which adds another mechanism. Require a name-reuse trace and unauthorized-ID tests
before deciding. Do not claim that exposing an ID currently grants access or that this change
alone fixes stale revisions.

## D13 — Treat default insertion as a language feature with a measurable cost

**Evidence.** INS-019 gives argument, input default, template value and Type-schema default
precedence. SCH-008/009/010 govern recursive insertion and where defaults may appear; OPS-026
normalizes before validation and idempotency comparison. REV-006 intentionally keeps omitted
control inputs unchanged.

**Critique.** A user must distinguish omission, null, an empty map, a default map, a destination
literal and nested property defaults. Defaults alter equality as well as convenience. These
are carefully chosen semantics, but they are more than using JSON Schema as a validator.

**Subtraction experiment.** Compare the current model with explicit whole-input defaults and
template configuration literals, without recursive insertion into supplied values. The smaller
model loses reusable deep defaults and makes some documents longer. Removing only template input
defaults is another possibility, but duplicates fallback values across destinations and loses
their direct input-schema validation unless a new rule restores it. Neither is an automatic win.

**Recommendation.** Keep the current semantics until this comparison demonstrates a net gain.
Meanwhile require executable normalization cases covering absent versus empty maps, defaults
through references, array elements, conditional schemas and control omission. Repeated calls
must compare the same normalized value. A canonical resolved view can expose the result without
adding a new authoring language; it does not reduce the engine's semantic obligations.

## D14 — Keep standard schema machinery; question special restrictions individually

**Evidence.** The accepted principles review widened the schema profile because a smaller
profile required handwritten checks. CTR-012 separately restricts Operation schema roots to
four keys. The general profile can express alternatives that an Operation's root cannot.

**Scenario.** An operation accepts either a path or a URI, but not both. The author can
express that relationship in a nested object using the profile, yet cannot put the same root
relationship in the operation's schema. Wrapping the input solely to access standard composition
adds structure without adding user meaning.

**Recommendation.** Keep the broad standard validator; do not replace it with a bespoke tiny
schema dialect. Compare allowing the existing profile at ordinary Operation roots while
retaining an object root. Treat controls separately: their partial-update rule deliberately
requires only `revision`, and arbitrary root constraints could require omitted inputs indirectly.

This reopens principles-review item 16a, not item 20's general adoption of standard vocabulary.
It needs wire-schema and control tests, including finding 6-1-04's reference cases. Broader root
schemas reduce a Conduit-specific exception but add valid inputs implementations must handle;
the tradeoff should be stated rather than called a mechanical simplification.

## D15 — Provider reuse is an integration claim, not zero infrastructure work

**Evidence.** PRV-002 and PKG-030 run runtime, machine and package-source provider servers
locally on the engine's own runner. Their remote services remain behind local adapters.
PRV-021 requires durable handling of delayed stops; PRV-022/025/028 specify identity and gates.
PRV-026 supplies provider restart behavior. These are meaningful obligations even when standard
tools supply the underlying process, machine or transport.

**Critique.** Moving behavior behind a provider is good architecture but does not remove its
implementation, deployment and operational cost. A “thin adapter” claim is unproven until the
adapter handles startup, secret access, restart, lost machines, outstanding stops and gate
configuration. Avoid equating an existing backing API with an already-conforming provider.

**Recommendation.** For A63, record what each provider implements itself, what it delegates,
what persistent state it needs and what the operator must configure. Use a second independently
backed provider to find contract assumptions. Prefer tightening a contract over adding a
Conduit-specific supervisor or distributed subsystem whenever an existing component can fulfill
the obligation. If the contract forces substantial custom infrastructure, bring that evidence
back to ruling 40's principle rather than hiding it in a package directory.

## D16 — Define what swapping a provider preserves

**Evidence.** PRV-006 fixes runtime identity for an instance's life; PLC-009 fixes placement.
Store locators are schemas of the selected secret Resource Type (SEC-005). Package sources
claim schemes under PRV-013/016/017. S10 asks for providers to be swapped.

**Critique.** Swappable contracts establish alternative implementations of operations, not
automatic migration of live runs, credentials, configuration or stored handles. Calling a
provider “swapped” after rewriting all templates and access would show less portability than
repeating the same relationship intent with only provider-specific configuration changed.

**Recommendation.** Define the S10 comparison before running it: same user tasks, same
intended access, same results and attribution; enumerate allowed provider locator/configuration
edits and required instance replacement. Record changed documents, downtime, recreated Conduits
and state migration. Do not require live migration that v0 never promised. Do require that a
second provider can satisfy the contract without provider-specific branches in the relationship
language. The optional immutability proposal for package-source scheme claims in finding
6-1-05 should be assessed against this explicit replacement cost.

One concrete artifact boundary belongs in the test: PRV-007 allows a relative package-contained
command such as `bin/files-server` only on the engine's own runner. Changing that example's
runtime to Docker is not enough; a suitable pinned `package` item is the current portable route,
and PRV-012 leaves supported registry/transport kinds to the runtime contract. Keep convenient
local commands for development, but use provider-ready artifacts when demonstrating substitution.

## D17 — Keep package installation explicit, but make dependency preparation usable

**Evidence.** NAM-006 pins version and digest; NAM-014 resolves a dependency through its own
package manifest. PKG-006/028 reject missing dependencies and install none automatically.
SUB-008 distinguishes project submissions by digest; SUB-009 fixes replacement behavior.

**Critique.** These rules aid reproducibility and prevent registration from silently fetching
dependencies; setup remains a separate explicit action. They also make “use this package” a
dependency-ordering task before the author can describe the relationship. Several identities
coexist: package release, archive digest,
contract `.v1`, project address, submission digest, object name and object ID. Each has a reason;
their combinations need to be visible at the right stage.

**Recommendation.** Have tooling prepare a reviewable, dependency-ordered set of explicit
installation requests with pinned artifacts. Registration remains non-fetching. Show a package
upgrade as a change to future resolutions unless instances are explicitly replaced. Explain
that adding a capability operation may require an implementation selection in newly resolved
templates, while explicit Conduit operation lists do not silently grant that new operation.

Do not add version ranges, implicit dependency installation or aliases simply to shorten the
first example. First measure author edits using well-packaged dependencies. A package author
can absorb repetition without requiring every end user to become one.

## D18 — Management routing should be a single checkable decision table

**Evidence.** ACC-033's `then` alone contains 406 whitespace-delimited words. It distinguishes
management, access, a Resource's Pool and the instance itself, and changes reconciliation and
inspection routing after retirement or removal. The same action name is intentionally reused.

**Critique.** The routing has reasons, especially ownership and recovery after a Pool is gone.
But “give this actor permission to manage that thing” requires understanding a decision tree
that is not obvious from the thing's own Conduits. It is a major source of avoidable user and
implementer error.

**Recommendation.** State one normative decision table, preserving current answers, and derive
targeted cases for every row and state transition. Explanatory tools should identify the required
authorization target when the caller is permitted to learn that information. Avoid disclosing
otherwise hidden objects merely to improve an error. If evidence later favors one uniform
management target, consider that as a separate behavior redesign with its ownership tradeoffs;
do not flatten the current rules in a “clarifying” edit.

## D19 — Count durable invocation state as a product cost

**Evidence.** OPS-008 keeps invocation state for every invocation, keyed or unkeyed, separately
from records: “No purge touches that state.” REC-004 also excludes retained outputs from record
purge; DL10 controls output retention. A successful old request can consequently be known while
its outputs are unavailable.

**Critique.** Permanent deduplication knowledge is defensible, but permanent metadata for all
unkeyed successful calls is a larger commitment than a user may infer from “purge records.”
It affects storage, backup, recovery and deletion expectations. There is no basis here for
claiming a measured capacity failure or a particular privacy-law violation.

**Recommendation.** Measure persisted bytes per keyed, unkeyed, failed and unknown invocation
in the reference implementation, and project representative A14 workloads using stated rates.
Distinguish record deletion, output expiry and invocation identity retention in the product.
An owner proposal could retain keyed deduplication state and unresolved invocations while
compacting older unkeyed terminal state. That would change old-ID inspection/reconciliation
behavior and needs an explicit contract. Never silently add a key TTL that allows an old
effect to be executed again.

## D20 — Put the actual trust and availability model beside the first example

**Evidence.** ACC-027 reports all v0 restrictions as mediated. The secret-limit section
explicitly trusts the engine's operating-system user. IFC-022 admits the credential-reading
exception, and PRV-025's gate does not confine the consumer. ACC-036/037 put access decisions
at the current control-plane state and fail closed when that state cannot be read. PLC-023
distinguishes unreachable from gone.

**Critique.** Users can read “revoked,” “gated” and “credential never handed over” as a stronger
isolation promise than the current deployment offers. Conversely, demanding isolation for
every local first use would burden the stated solo-developer audience. The present scope is
legitimate; its consequences must be visible at the point of choosing a runtime.

**Recommendation.** Show a local mediated deployment and an isolated-runtime deployment with
the same relationships. State which credentials an Agent process can read and what a gate
protects. S3 should demonstrate both safety and usability under a partition: no new dispatch
without current authorization, clear unavailable results, and honest unknown effects. Measure
the availability cost of the central decision point before proposing v1 local enforcement.
Do not delete multi-machine scope because S1 is easier, or claim v0's gate is already the
optional confinement promised for later enforcement.

Also distinguish two remote application hosts from a movable or multi-part engine. DL17 makes
the latter a configured engine choice. A fixed control plane with work on two hosts can test
the accepted S3 semantics without making distributed-engine failover a prerequisite. Preserve
the additional obligations for engines that claim that capability.

## D21 — Separate author reading from implementer reading, without hiding semantics

At the reviewed commit, whitespace-token counts are:

| Surface | Words |
| --- | ---: |
| `spec/README.md` | 6,215 |
| `spec/definitions.md` | 24,141 |
| Nonbinding overview | 6,719 |
| All promise `when`, `then`, `observable` text, excluding YAML comments | 78,627 |

There are 452 promises: 202 language, 207 convention and 43 interface. Counts are a reading
burden measurement, not proof that a rule should be removed. The entry point spends substantial
space on history before explaining the present design; many glossary rows hold algorithms and
exceptions. “Few shared conventions” names seven topics, not a small behavioral specification.
The language's configuration identity does not make those behavioral obligations disappear.

**Recommendation.** Preserve the historical records and stable promise IDs. Put present rules
first in reference entry points; give end users, contract/provider authors and engine builders
different reading paths. Supply a compact glossary whose longer semantic explanations link to
the governing promises. Keep one normative source for each rule; avoid three paraphrases that
each acquire exceptions. Treat `observable` as part of the obligation review: several important
decisions live there rather than in `then`.

The current valid examples exercise shapes and use invented artifacts. Preserve them, and add
the complete executable S1 journey when the planned model/providers make it possible. A
formatter or authoring convention can favor one readable reference form among semantically
equivalent forms in the same context, without adding new syntax. Restricting submitted-project
`from: '#name'` is a possible low-priority synonym removal,
but would reduce source portability into packages; do not change it merely to reduce spelling
count. Test whether a fresh author can predict setup, access, change and recovery outcomes from
the author path alone.

## D22 — Judge progress by cross-rule evidence and usefulness, not finding counts

**Evidence.** The tracker already withdrew the two-clean-round gate because interactions kept
producing findings. The new counterexamples again cross boundaries: protocol errors and effect
semantics; filters and cached delivery; control pointers and omitted inputs; schemas and wire
publication; provider selection and controlled mutation.

**Recommendation.** Continue the accepted order: decide the relevant findings, use S1 as the
serial tracer through the reference model and vectors on first providers, then second providers
and S10, then the wider scenarios, model checking and independent builds. Target the same
interactions in small state machines. Do not postpone safety-case vectors until after a large
implementation, and do not spend another round primarily polishing unaffected prose.

Conformance is necessary but not sufficient for usefulness. Add an observed author task to the
existing scenarios: how many concepts, source files, manual grants and external adapters are
needed; how often the user must inspect history; how many recovery steps require a human; and
whether the restrictions actually express the intended collaboration. The first end-to-end
test should be allowed to falsify a simplification proposal in this report. Agreement between
reviewers is not substitute evidence.

## The four open points

The tracker expressly allows these to be reported. They are included here for a complete
disposition, not counted again as new defects.

| Open point | Assessment of current text | Recommendation and test |
| --- | --- | --- |
| Type default for a hook not listed in `requires` | TPL-004 plus TPL-017/018 most strongly supports ignoring it unless selected/required. CTR-004's rejection of an unresolvable default does not clearly reject every unused default. | Document the selection result and diagnose an inert hook default. Explicit rejection is an owner alternative. Test a Type default with no `requires`, then the same Type requiring the hook. Do not present two equally strong interpretations where the hook rules favor one. |
| Repeated Package argument name, such as `-e` | MAN-010 rejects a binding name that matches more than one argument. Behavior is defined; the expressiveness limit is real. | Exercise a real candidate package in S7/A63. Prefer a supported packaging convention if it suffices. Add occurrence selection only if needed, defining exactly which pinned argument it binds and preserving literal arguments. Do not introduce interpolation to solve a narrow indexing problem. |
| Machine-loss record cause | PLC-024/REC-002 require a cause, but the record schema does not standardize its field. Additional members and engine-selected event types can carry it. | Settle a portable machine-readable cause with the already-planned inspection/vector work. Test an unsolicited loss without inventing a caller or invocation. This is an interoperability gap, not evidence the cause cannot be recorded at all. |
| Late onward request from an earlier hook attempt | OPS-028 explicitly attributes it to the latest caller. It is a documented v0 accountability compromise, not unspecified behavior. | Preserve earlier attempt records, expose the limitation in the relevant inspection/explanation, and include a regression trace with two callers and delayed delivery. Ask whether the compromise is acceptable for the initial multi-machine workflow. Any attempt-identity or local-enforcement repair needs a decision; do not promise exact attempt attribution before it exists. |

## What to cut, retain or investigate

This table distinguishes semantic subtraction from moving complexity into tooling or packages.

| Candidate | Recommendation | What is given up or still owed |
| --- | --- | --- |
| Operation-level implementation defaults | Favor comparison toward removal (D06) | More Type mappings; no operation-selected executable fallback |
| Template control input narrowing | Prefer exposing effective constraints; compare removal (D07) | Interface transformation, or specialized contract proliferation |
| Original-project-only context during replacement | Compare replacement against a fresh explicit context (D11) | Expanded request inputs and digest/reference lifetime cases |
| Actor-only ban on resolved IDs | Favor reconsidering the ban (D12) | Must retain all authorization checks and define removed-ID behavior |
| Recursive schema default insertion | Measure before changing (D13) | Removing it sacrifices reusable deep defaults and changes equality |
| Ordinary Operation-root schema restriction | Compare relaxation separately from controls (D14) | More wire cases; controls still need partial-update semantics |
| Mutable package-source scheme claims | Prefer immutable claims if live changes lack a scenario | Provider replacement instead of mutation; must cover indirect controls |
| Permanent state for every unkeyed completed invocation | Measure and propose compaction only with explicit semantics (D19) | Old invocation-ID lookup and reconciliation may change |
| Five families, ownership, explicit lifecycle, unknown outcome | Retain | Necessary distinctions remain in the author and implementation model |
| One-run rule, one capability per Conduit, no expression language | Retain pending contrary scenario evidence | Some composition stays in implementations; some extra Conduit documents |
| Automatic apply, rollback, key inference, general policies/graphs | Do not add to make examples look shorter | Tools may prepare explicit actions; effects and uncertainty remain visible |
| Standard schema vocabulary and existing protocol machinery | Retain, with precise transformations at boundaries | Adoption still needs interoperability cases and honest integration limits |

None of these rows silently removes accepted v0 scope. In particular multi-machine work,
multiple owners, provider substitution and agent-driven management remain preservation
constraints. A scenario showing no use for a feature makes it a candidate for the owner's A18
process, not permission for a reader to delete it.

## Proposed evidence for the next stage

These are concrete cases to turn into the accepted vector/model formats. Expected assertions
below are recommendations where the governing decision is still open. They were not executed
against an engine.

| Case | Setup and perturbation | Assertion or question to settle | Traceability |
| --- | --- | --- | --- |
| E01 | Plain effectful server changes external state, then returns generic -32021; caller repeats its key | Do not infer no effect from the protocol error; conservative outcome unknown prevents blind redispatch | Finding 01; IFC-010/045, OPS-018/024 |
| E02 | Successful keyed read behind deadline filter; repeat after deadline with identical arguments | Old filter identity is not current permission; no cached disclosure without an authorized replay rule | Finding 02; OPS-009/034, ACC-036, IFC-019 |
| E03 | Two control inputs target `/limit`; stored 0, supply only `a: 1` | Reject overlapping destinations or explicitly resolve the incompatible omitted-input rule | Finding 03; CTR-006, REV-006/007 |
| E04 | Control targets `/options` and `/options/limit` | Same rule handles ancestor overlap, independent of map order | Finding 03; INS-010 analogy |
| E05 | Publish a contract with local `$defs`, two colliding local definition names, then recursive contracts and `$ref` siblings | Finite standalone wire schema preserves validation and all reference targets | Finding 04; IFC-033, SCH-015 |
| E06 | Two providers concurrently change claims to one scheme; one reply becomes unknown | Unique ownership survives; observable refusal and reservation/replacement semantics are explicit | Finding 05; PRV-013, REV-012, DL17 |
| E07 | Give agent a filtered read and a separate bound covering the same operation; it opens an unfiltered Conduit | Show current authority accurately; determine whether the user intended to grant it | D03; ACC-029/030 |
| E08 | Request permission to instantiate only reviewer template X | Demonstrate current broad grant or broker; measure the narrow workflow's cost | D04; ACC-033 |
| E09 | Ordinary client reissues identical effect without a key after a lost response; key-preserving client repeats the original key | Two deliberate semantics, no equality-based heuristic; qualification makes difference visible | D02; IFC-023, OPS-011 |
| E10 | Advertise control on max-50 target with max-100 contract; ask for 75; then remove control from template map | Discovery should predict stable target constraints under chosen design | D07; TPL-010/011/013, IFC-033 |
| E11 | Edit a local operation and implementation, then replace its template while old instance exists | Show present original-context behavior and compare proposed fresh-context replacement | D11; SUB-009, INS-020 |
| E12 | Inspect object A, remove it, reuse its name for B, then perform follow-up action | Demonstrate name/ID choice; proposed ID path must neither retarget nor grant authority | D12; OBJ-009, LIF-031 |
| E13 | Rotate service secret with both agents active, replace service, fail partway through access restoration | No secret copied into Agent runs; honest downtime and explicit repair | D08/09; SEC-014, CON-008 |
| E14 | S1/S2 with each provider swapped | Same relationship intent; enumerate rather than conceal necessary provider-specific edits | D15/16; A63, S10 |
| E15 | Machine provider reports unreachable, then gone; partitions occur before/after effect and before/after revocation | Distinguish unavailable, unknown, retired and stopped; no unauthorized new dispatch | D20; PLC-023/024, ACC-036/037 |
| E16 | Omitted input, null, empty map, nested default, array element and control omission; compare keyed repeats | Deterministic normalization with no accidental new effect or changed control setting | D13; SCH-008/009/010, OPS-026, REV-006 |
| E17 | Agent sees cached tools after revoke, protocol-level refusal, and tool-level failure | Record exactly what client and model see; do not infer model visibility from wire response alone | D01/02; IFC-012/025/038 |
| E18 | Retire owner Pool; inspect/reconcile Resource; remove instance; reconcile again | Authorization follows each ACC-033 row, including state-dependent changes | D18; ACC-033 |
| E19 | Purge records and expire retained outputs, then repeat old keyed request and inspect unknown invocation | Purge does not cause a duplicate effect; explain retained state and missing outputs | D10/19; OPS-008/034, REC-004 |
| E20 | A's hook reply is lost; its unknown outcome is reconciled failed; a new attempt serves B; A's delayed onward request arrives | Preserve current documented behavior in test and make attribution limitation explicit | Open OPS-028 point |

For usefulness, compare the same S1/S2 tasks with a deliberately simple manually configured
baseline using the same agents, service and backing tools. This is a proposed experiment, not
a claim about another product. Measure duplicated credentials/configuration, per-agent access
changes, inspection effort, provider-specific code and recovery interventions. A language that
adds setup work can still win if changing and auditing relationships becomes materially easier.
It should demonstrate that gain.

Suggested success criteria for the first author trial: a competent reader can complete the
checked S1 path without reading ruling history or editing a provider; the S2 restrictions differ
as intended without copying the shared service credential to agents; closing one agent's access
leaves the other working; the user can identify and resolve uncertainty without guessing that
an effect failed. Record observed time and mistakes before setting numerical targets. These
criteria supplement the accepted scenarios; they do not substitute for conformance.

## Review coverage and limits

The initial pass read all 452 promise obligation texts, with selective supporting definitions,
observables, contracts and schemas. This expansion used three separately tasked reviewers for
authoring/simplicity, usefulness/providers, and adversarial checking of the six findings. They
shared this task's context; they are not three independent cold rounds. Their recommendations
were integrated and checked against the current text. No new finding was added merely to
increase a count.

| Area | Evidence used in this report | Limit |
| --- | --- | --- |
| Objects, templates, instantiation, naming, packages, manifests, submissions | Governing promises, reference schemas, current project/files fixtures; replacement and default examples | No full resolver or package installation executed |
| Capabilities, operations, requirements, controls, schemas | Contract promises, control restrictions, current schemas and core exports; two small checks in the findings report | Not every allowed schema construct independently tested |
| Access, ownership, lifecycle, revisions, outcomes, records | Cross-rule traces, management routing, current definitions and core operation/result shapes | Analytical traces, not runtime or model-checker results |
| Providers, placement, first start, secrets | Provider promises, explicit trust limits, ruled direction and S1–S10 plans | No provider, network partition, secret rotation or migration run |
| MCP interface | Current interface promises; primary sources cited in the findings report; adversarial recheck of missing-capability semantics | Not a complete protocol conformance suite or real-client compatibility test |
| Simplicity/usefulness | Measured text surface, complete example inspection, concrete user journeys and alternatives | No user trial; reduced reading burden and improved usability remain hypotheses |
| History/status | Rulings and prior decisions consulted only for scope, provenance and counterarguments | Historical documents do not supply missing normative behavior |

The static repository checks validate links, names, document/schema consistency and whitespace.
They do not validate the proposed design changes or certify safety. Known absences of a model,
vectors and an engine remain known prerequisites, not newly scored defects. The four open
points, accepted secret trust boundary and stdio bridge limit are not inflated into discoveries.

## Handoff to Claude or the coordinating session

1. Adjudicate each of the six numbered findings separately, including the counterarguments to
   04 and 05. Resolve the safety contradictions regardless of their labels. Log accepted,
   rejected or clarified with a reason; this reader's report is not that log.
2. Decide whether the first useful product requires filtered delegation and narrowly authorized
   peer provisioning, or whether explicit broad trust/broker operations are sufficient. That
   choice determines whether D03/D04 are documentation limits or language work.
3. Compare the strongest simplifications: Operation defaults, effective control discovery,
   local replacement context and stable-ID follow-up requests. Do not batch-accept them:
   their costs differ and some reopen ruled choices.
4. Turn the relevant E-cases into the existing vector/reference-model program. Start with the
   safety boundaries and the serial S1 tracer; use S2 and the accepted provider-swap stage to
   judge usefulness. Preserve the independent-build gate and fresh-builder isolation.
5. Publish the actual tested author path, qualified client/server combinations, and the cost
   of change and recovery. Revisit the remaining subtraction candidates only with that evidence.

The recommendation is to preserve the relationship model, reduce overlapping mechanisms where
comparison supports it, and make the next milestone a working demonstration of useful restricted
sharing with honest failure recovery. Neither a smaller word count nor another long list of
accepted prose fixes would establish that result.
