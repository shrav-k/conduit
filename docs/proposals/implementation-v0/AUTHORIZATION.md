# Policy and Grant contracts

Status: semantic proposal for I4 review, 2026-11-01. The user directed us to formalize Policy and
Grant as essential concepts beneath the five object families, for Agents and Interfaces alike.
Independent policy is accepted. This document develops their structure without selecting exact
YAML/MCP fields, an evaluator, credential format or storage engine. New choices below are proposals;
the [design](../../DESIGN.md) retains accepted behavior and [I4](REVIEW.md#i4--local-transactional-authority)
is the ruling queue. [Research](../../research/AUTHORIZATION.md) provides sources and alternatives.

## 1. Place in the model

| Concept | Responsibility | What it does not imply |
| --- | --- | --- |
| Policy | Authoritative rules governing access, administration and delegation for applicable actors/targets. | A running instance, a grant merely because a file was authored, or permission inferred from a model instruction. |
| Grant | A particular issued authorization for an identified subject, target/capability and permitted extent, with recorded provenance and lifetime/dependencies. | A bearer secret, an OAuth token, perpetual permission, or the union of unrelated access paths. |
| Requirement | A reusable typed guarantee with applicability, timing and endpoint duties. Policies/grants can reuse applicable restrictions. | All requirements being authorization rules, or satisfying a restriction creating positive authority. |
| Plugin | Contracts and implementation bindings for management, evaluation and enforcement operations. | Every plugin becoming an issuer or implementing its own permission server. |
| Operation / graph | A callable action / composition of actions that may request access, delegate or invoke capabilities. | An executable declaration granting itself authority or freezing authorization for all later graph nodes. |
| Conduit | An instantiated relationship carrying concrete participation, routes, applicable requirements and access evidence. | Ownership of every governing policy or independent invention of permission. |

Policy and Grant are protocol concepts with data/lifecycle contracts. They do not extend the five
families or need their own template-to-running-instance hierarchy. Published definitions, authorized
policy records and issued grants must remain distinguishable. Installing a plugin or expanding a
template can make definitions available and request access; it does not issue a grant.

## 2. One authorization model for managed actors

An Agent and an Interface can both request an action, receive authority, administer permitted
policy and delegate when authorized. Their defaults and identities differ; their requests must use
the same semantic checks. V0 root administration remains broad and owner-operated. Agents do not
inherit it from the OS account, subscription, frontend or user who started them.

The protected target may be any family's capability: inspect an Interface, pause an Agent,
provision through a Pool, invoke a Resource or join/manage a Conduit. Resource-to-Resource activity
also needs attributable authority. The word target here is not limited to the Resource family.

Proposed request identity distinctions:

| Identity/context | Example | Source of truth |
| --- | --- | --- |
| Managed actor | Agent A, Interface E, or a Resource making a dependency call. | Authenticated runtime/endpoint binding, not a claimed name in operation arguments. |
| Acting principal / delegated origin, when applicable | Local owner using E; a coordinator that authorized A. | Verified caller/delegation context. Historical origin alone is not current permission. |
| Execution/frontend attribution | A's current execution, or the CLI attached to E. | The responsible service's invocation context. |
| Selected target and path | Linear Resource L through participation C-A. | Resolved stable identities and the chosen route. |
| Provider service identity | The Pool's shared Linear account. | The authenticated provider binding. It is distinct from A or the local owner. |

These distinctions need not become new registry objects or force every call to carry an arbitrary
identity chain. Exact principal representation, trusted claims and autonomous Interface identity
remain to be reviewed. No actor can gain more rights by supplying an on-behalf-of claim.

## 3. Proposed Policy structure

This is a semantic field inventory, not final keys or a requirement for a new universal policy DSL.

| Part | Meaning |
| --- | --- |
| Identity and revision | Stable policy identity, checked update revision and interpretation/contract version. Names are descriptive. |
| Governing authority and applicability | The authority responsible for the record and the actors, targets, capabilities or relationships it governs. Pool ownership alone does not grant access. |
| Access and administration rules | What uses or management/delegation actions may be authorized, and any governing restrictions. Direct-permit versus grant-required use is an open ruling. |
| Typed conditions/requirements | Versioned references with validated parameters, applicability and needed facts. Reuse compatible requirement contracts rather than duplicate workspace/path semantics. |
| Delegation boundary | What authority may be issued, to whom, over which targets, and whether further delegation is possible. |
| Provenance and current validity | Who authorized the change, operation/evidence references and the current authoritative revision/availability. A package source is not an authority. |

Policy changes are managed state changes, separate from editing a fixed instance definition.
The governing service must authorize and check updates. A change may affect later managed requests
under the accepted live-restriction rules. It cannot silently install a new evaluator, replace an
endpoint plugin or drop a requirement the current path cannot enforce.

The decision must include every applicable governing restriction. A policy on one selected path
does not automatically become global; actor-wide/target-wide limits apply according to their
declared applicability. Policy aggregation/effect syntax remains open. We should not inherit
additive RBAC, ordered-rule evaluation or an engine's error behavior merely by selecting a library.

## 4. Proposed Grant structure

| Part | Meaning |
| --- | --- |
| Identity and revision | Stable issued record and checked management version. The ID is a reference, not a credential. |
| Recipient | The identified actor/subject receiving authority. Human principal and Interface endpoint distinctions remain explicit where needed. |
| Target and access extent | Concrete targets/capabilities, permitted operations and associated parameter/resource restrictions, preserved together. |
| Participation/path binding | The logical attachment or other declared authorization context in which the grant may be exercised. Service-access grants already follow recipient participation. |
| Issuance authority and attribution | Who requested/issued it, the authorized operation and relevant policy/authority evidence. Do not confuse original issuer with current recipient. |
| Continuing dependencies | Any governing policy or parent authorization whose validity/restrictions must still hold. Historical source references alone must not silently imply live dependency. |
| Delegation bounds | Whether and how the recipient may issue further grants. Ordinary use does not imply onward delegation. |
| Lifetime and disposition | Applicable start/end conditions and withdrawal information. Participation ending differs from a transport outage. No mandatory expiry interval is selected here. |

Issuance is an authorized operation, not an authored object being trusted on import. The initial
implementation can keep a grant as local authoritative data; it need not mint a cryptographic token
or create a Resource per grant. A grant can support many requests. Retries of an issuance operation
use its recorded outcome instead of accidentally issuing independent authority twice.

Accepted lifetime rules continue: issuer/frontend liveness does not govern recipient access;
detachment ends that participation's access; withdrawal/current restrictions apply; exported
configuration does not carry live grants. New instances/identities do not inherit old bindings by
name. Old issuer attribution stays historical after Interface replacement.

**Still open:** exact parent-dependency and delegation-attenuation semantics. Revoking a parent
grant, changing the issuer's personal permissions and stopping the issuer are different events.
We should decide which dependencies remain live rather than infer cascade behavior from provenance.
Previously discussed knowledge-sharing rules do not automatically settle every service grant.

## 5. Decision and enforcement contract

Proposed normalized authorization input:

- Verified actor/principal context and concrete target/capability operation.
- The selected complete route/participation and provider identity where relevant.
- Typed arguments or derived facts required by the applicable rules, with trusted origins.
- Relevant grant/policy revisions and current relationship/requirement state.
- Invocation identity and any configuration preconditions for the operation itself.

Proposed result: distinguish permission established, refusal and inability to establish permission,
with scoped reasons, relevant authority revisions and the remaining or satisfied enforcement
obligations. Exact enum/wire schema remains open; detailed diagnostics may be extensible. A client
receiving a decision cannot reuse it as an access token. Explanation access must not reveal policy
or target details outside the viewer's authority.

The endpoint responsibilities are already partly fixed: selected plugins declare requirement
support and duties; Conduit records which implementation is responsible. Policy evaluation can
identify obligations, but the request/result path must fulfill them at the declared points. Hiding
a tool from an Agent's catalog alone is not enforcement of a server-side restriction.

For one request, admission requires a complete positive authorization basis plus all applicable
restrictions on that path. Separate paths remain alternatives; read on Euphony plus update on Demo
cannot yield update on Euphony. Governing actor/target restrictions still apply. Whether every
positive basis must be an issued grant or a policy may directly authorize use is the next ruling.

Unverified required access/checks block the affected behavior. Do not turn an evaluator error,
missing policy, unsupported restriction or cached grant into permission. Preserve the reason so
unavailable differs from an actual refusal. The accepted no-offline-grants rule permits verified
local authority; it does not require a network round trip when the current authority is local.

Authorization does not promise remote-effect atomicity. Record the admission basis; distinguish
withdrawal before dispatch from an already-dispatched effect and recheck at required delivery
points. The exact ordering/fencing mechanism across services must be designed and tested. Do not
claim instantaneous global revocation, cancellation of completed effects or erasure of delivered
native context. Runtime supervision can continue while new requests wait for current permission.

## 6. Resource, plugin, graph and MCP placement

**Proposed placement:** a Pool-owned authorization-management capability exposes policy/grant
inspection and authorized changes. It may initially be part of the existing system management
Resource, or a separate Resource if its responsibility/lifecycle warrants that boundary. This
placement remains a ruling; either way, policy remains independent of its administrative Interface.
One capability can manage many records; the designated service/store owns authoritative facts.

The initial trusted authority must be selected by the system's establishment contract. An arbitrary
Resource advertising policy evaluation cannot approve its own use or confer root authority.
Initial owner recognition, first records and management-path repair remain I4 startup questions.
Policy records are not ordinary knowledge notes: copying, searching or sharing a note cannot confer
administrative authority or bypass the policy-management contract.

Proposed operation responsibilities, with names deferred:

| Surface | Responsibilities |
| --- | --- |
| Administration | Inspect policy/grants, submit checked policy changes, issue/narrow/withdraw grants and inspect their outcomes. |
| Evaluation | Resolve current authoritative access facts for a concrete request and return the typed decision/evidence. |
| Enforcement | Bind authenticated identity/path, apply declared checks at dispatch/delivery and record outcomes. |
| Observation | Explain authorized access, dependencies, failed checks and incomplete coverage through the Interface. |

An ordinary operation plugin author declares operations and requirement duties and consumes trusted
invocation context; the runtime supplies common authorization plumbing. A policy plugin implements
a specific decision/management contract when selected. Graphs call operations with current authority
at each applicable step; a successful setup node is not a lasting blanket authorization.

MCP remains the invocation/discovery base. The Conduit profile carries the managed context needed
for these contracts; HTTP authentication uses the relevant MCP authorization mechanisms. Native
provider tools remain exposed with permitted schemas. Provider tokens, Conduit grants, MCP session
identifiers and operation receipts remain different things. Shared upstream credentials do not
collapse Agent identities or justify forwarding the caller's token to another audience.

## 7. Worked delegation across Interface and Agents

This is a semantic example, not final YAML or an adopted permission language. Assume explicit
grants supply ordinary access in this example; the general positive-basis rule is still open.

| Stage | Authority and behavior |
| --- | --- |
| Root setup | The verified owner-operated Interface receives broad management access under the initial system contract. A copied template is insufficient. |
| Euphony administration | Root authorizes Interface E to administer a defined Euphony access/delegation capability. Its future narrower reach is illustrative; broad v0 Interface defaults remain unchanged. |
| Coordinator | E authorizes Agent A to read Linear and delegate a bounded subset to worker Agents. That delegation ability is explicit, not inferred from read permission. |
| Worker | A requests a grant for Agent B: read-only Euphony access on B's participation, without further delegation. The issuing authority checks A's actual delegation boundary. |
| Provider call | B's concrete read request passes the selected grant, governing policy and workspace/operation requirements; the Linear Resource uses its shared provider identity. |
| Forbidden broadening | B cannot create write authority or redelegate merely by submitting wider YAML or calling the management tool. |
| A stops | B's participation does not end just because A stopped. Explicit withdrawal or a change in a declared continuing dependency is a different event. |
| Root replaced | The new Interface establishes valid access to the existing policy; it does not copy grants out of the old template. |

Every issuer may also be a protected target in another operation. Agent-to-Agent controls, Pool
provisioning and Resource dependency calls follow the same request/basis/check/result structure.

## 8. Review sequence and conformance evidence

Continue within I4, not a new decision ledger:

1. **Positive authority:** require a recorded grant on ordinary managed participation, including
   broad root management, or allow direct policy permits as an additional authorization basis?
   Recommend grants for the first version: it gives all actors inspectable participation authority
   without a new grant per request. Policy still governs issuance and current use.
2. **Delegation and dependencies:** choose the allowed narrowing behavior, delegation ceilings and
   live parent/policy dependency rules. Custom constraints must not claim subset relationships
   they cannot establish; a general solver is not assumed.
3. **Policy composition and failure:** define applicability, positive/negative rule interaction,
   incomplete information and trustworthy decision inputs under existing whole-path restrictions.
4. **Placement and persistence:** settle the management Resource capability, first authority and
   startup/repair boundary, then storage interface/engine. The root-owner initial grant is a
   concrete initialization problem, not permission for runtime self-granting.

The final I7 pass chooses policy/grant schemas, package publication, references, operation names,
dynamic MCP-operation matching and retained implementation bindings. Registering new contract
types and activating authoritative policy remain different operations. No Cedar/OPA/token engine
or public policy-programming language is selected by this proposal.

Candidate additions to Experiment 002, pending scope review:

- Exercise equivalent capability use and administration through Interface and Agent actors.
- Author policy/requirements, issue grants and show definitions alone confer nothing.
- Test permitted delegation and rejected broadening, preserving target/operation relationships.
- Inspect grant lineage and replace/stop the issuer without confusing that with withdrawal.
- Withdraw access, sever policy reachability and trigger a required-check failure; show distinct
  evidence while none authorizes a new affected operation.
- Prove an existing provider credential cannot substitute for the requesting Agent's grant.
- Trace a decision to actual endpoint enforcement; include a malicious direct invocation that
  bypasses discovery but is still refused under the same restriction.

These are proposed acceptance cases, not executed tests or authorization to start implementation.
The first proof should establish the common contract using a small implementation before investing
in helper libraries, a new policy DSL or distributed authorization infrastructure.
