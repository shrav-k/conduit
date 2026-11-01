# Policy and Grant contracts

Status: semantic proposal for I4 review, 2026-11-01. The user directed us to formalize Policy and
Grant as essential concepts beneath the five object families, for Agents and Interfaces alike.
Independent policy, recorded participation grants and issuer-independent grant lifetime are
accepted. This document develops their structure without selecting exact
YAML/MCP fields, an evaluator, credential format or storage engine. New choices below are proposals;
the [design](../../DESIGN.md) retains accepted behavior and [I4](REVIEW.md#i4--local-transactional-authority)
is the ruling queue. [Research](../../research/AUTHORIZATION.md) provides sources and alternatives.

## 1. Place in the model

| Concept | Responsibility | What it does not imply |
| --- | --- | --- |
| Policy | Authoritative rules governing access, administration and delegation for applicable actors/targets. | A running instance, a grant merely because a file was authored, or permission inferred from a model instruction. |
| Grant | A particular issued authorization for an identified subject, target/capability and permitted extent, with its own participation lifetime and authorized administration. | A bearer secret, an OAuth token, a parent-dependent authority chain, or the union of unrelated access paths. |
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
| Acting principal, when applicable | Local owner using E. | Verified current caller context. An Agent's historical grant issuer is not its acting principal or a current permission dependency. |
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
| Access and administration rules | What authority may be issued and how its use is governed. Ordinary managed access requires a recorded participation grant; a direct policy permit alone is insufficient. |
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

### Policy and Requirement reuse — proposed refinement

The user asked whether Policy and Requirement should share or combine their evaluation machinery.
The portable policy-contract choice remains open. Recommend reusing the existing requirement
contracts and composition mechanism, with Policy supplying authoritative applicability and
grant-administration rules. This does not require a second general-purpose constraint language.

| Layer | Example | Responsibility |
| --- | --- | --- |
| Requirement definition | A versioned operation restriction or provider-project restriction. | Defines the guarantee, typed parameters, supported contexts and enforcement duties. |
| Reusable composition | `review-access` combines permitted read operations with an Euphony project restriction. | Reuses constituent meanings and support checks without creating authority. |
| Activated Policy | Authorized coordinators may issue review access to this service; applicable service access must remain within its project restriction. | Determines whom/what the rules govern and what grant administration is permitted. Installation requires current administrative authority. |
| Recorded Grant | Worker B receives review access on its selected Conduit participation. | Records actual issued authority and its extent/lifetime. A policy declaration alone cannot replace it. |
| Conduit requirements | This connection also limits returned results to ten. | Adds relationship guarantees/restrictions without broadening the grant or governing policy. |

These are explanatory names and values, not an approved core vocabulary or YAML syntax. The
coordinator must itself have a valid management grant permitting the administration request;
being named by a policy or able to read the service does not independently authorize issuance.
After authorized issuance, B's grant remains independent of the coordinator's later permissions.

**One extension mechanism:** a plugin package can publish the requirement definitions, reusable
compositions, evaluation operations and endpoint enforcement support together. The same supported
project check can serve a policy restriction, a grant's access extent and a Conduit filter; authors
need not redefine it for each. This does not mandate separate policy/requirement plugins, services
or processes. Independent implementations can still support the same versioned meaning.

**Contexts remain explicit:** checking a concrete provider request and verifying that a proposed
grant stays within an administration boundary are different tasks. Shared definitions do not
make a per-request checker a general subset prover. Support for each use must be declared and
verified; unsupported checks cannot be assumed successful. Exact administration-bound checks
remain a subsequent ruling, without requiring a universal solver.

**Evaluation and fulfillment remain distinct:** a plugin may decide whether an operation's
actual target satisfies a restriction; the endpoint must still prevent disallowed dispatch.
A result bound must be fulfilled before delivery, while retained-history guarantees need ongoing
Resource behavior. The same plugin can do both evaluation and enforcement, but a successful
boolean check alone cannot provide those effects. Plugins retain their declared endpoint duties.

Policy and Requirement therefore need distinct semantic roles without separate implementations
by default. A requirement may concern compatibility or durable delivery rather than permission.
An arbitrary template cannot activate a governing policy merely by including requirements. Current
authorization, grant checks and mandatory restrictions cannot be omitted as optional graph nodes.
Changing authoritative policy still uses its checked administration contract; configuring a
relationship's requirements follows its supported controls and fixed-definition rules.

The ruling to consider is a Policy as an authoritative composition of reusable requirements plus
grant-administration rules, using common contract/plugin machinery. Exact representation, engine,
rule-combination behavior and permission to activate a policy remain separate questions.

## 4. Proposed Grant structure

| Part | Meaning |
| --- | --- |
| Identity and revision | Stable issued record and checked management version. The ID is a reference, not a credential. |
| Recipient | The identified actor/subject receiving authority. Human principal and Interface endpoint distinctions remain explicit where needed. |
| Target and access extent | Concrete targets/capabilities, permitted operations and associated parameter/resource restrictions, preserved together. |
| Participation/path binding | The logical attachment or other declared authorization context in which the grant may be exercised. Service-access grants already follow recipient participation. |
| Administration evidence | A link to normal operation evidence where retained; no separate grant-provenance chain is required for authorization. |
| Current governing restrictions | Applicable policy and requirements constrain use independently of the grant's issuer/history; this is not a parent-grant dependency. |
| Grant-administration bounds | Whether and how the recipient may administer further grants. Ordinary use does not imply authority to issue them. |
| Lifetime and disposition | Applicable start/end conditions and withdrawal information. Participation ending differs from a transport outage. No mandatory expiry interval is selected here. |

Issuance is an authorized operation, not an authored object being trusted on import. The initial
implementation can keep a grant as local authoritative data; it need not mint a cryptographic token
or create a Resource per grant. A grant can support many requests. Retries of an issuance operation
use its recorded outcome instead of accidentally issuing independent authority twice.

Accepted lifetime rules continue: issuer/frontend liveness does not govern recipient access;
detachment ends that participation's access; withdrawal/current restrictions apply; exported
configuration does not carry live grants. New instances/identities do not inherit old bindings by
name. Available operation attribution stays historical after Interface replacement.

### Independent issued grants — accepted

The administrator must have current permission to issue, change or revoke the particular grant.
That check includes its target and permitted access extent; ordinary service use does not imply
authority to administer grants. The same rule applies to authorized Agents and Interfaces.

Once issued, a grant does not depend on the continuing rights, connection, execution or grant
history of its issuer. There is one issued-grant model; the proposed administrative-versus-derived
lifetime split is not adopted. Narrowing or ending the issuer's grant does not automatically narrow
or end grants it previously issued.

Any currently authorized administrator can revoke the grant; it need not be the original issuer.
Ending the recipient's related Conduit or logical participation ends that participation's access.
Ending only the issuer's unrelated participation does not. Closing a frontend or losing a transport
is not itself proof that the relevant logical participation ended.

Current governing policy and relationship requirements still apply to each use. Independence from
issuer history is not exemption from those current rules, and a stored grant is not an offline
authorization guarantee. This revises parent-chain grant proposals, not source-content dependencies
or actual Resource availability.

The normal operation record can retain attribution and outcomes for observability/recovery. No
extra parent-grant/provenance structure is required to authorize use. We do not revalidate the
original administrator's present rights on each request or require its participation to stay open.

**Example:** root authorizes coordinator A to administer read grants. A issues a read grant for B.
A later loses its administration permission or leaves. B retains its own access under current
policy until an authorized administrator revokes it or B's participation/Conduit ends. A cannot
issue another grant without current permission. A different authorized Interface can revoke B's
grant without consulting A.

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
cannot yield update on Euphony. Governing actor/target restrictions still apply.
**Ordinary managed use requires a grant
on the selected participation**, including root management. Policies govern issuance and current
use rather than supplying a grantless alternative. Authorized connection setup can establish a
grant covering many requests; it adds no separate user manifest or per-call token. The first root
grant still requires an explicit initial-authority contract.

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
| Observation | Explain current grants, governing restrictions, failed checks and incomplete coverage through the Interface. |

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

This is a semantic example, not final YAML or an adopted permission language. Recorded grants
supply ordinary managed access and remain independent of their issuer after authorized issuance.

| Stage | Authority and behavior |
| --- | --- |
| Root setup | The verified owner-operated Interface receives broad management access under the initial system contract. A copied template is insufficient. |
| Euphony administration | Root authorizes Interface E to administer a defined Euphony access/delegation capability. Its future narrower reach is illustrative; broad v0 Interface defaults remain unchanged. |
| Coordinator | E authorizes Agent A to read Linear and delegate a bounded subset to worker Agents. That delegation ability is explicit, not inferred from read permission. |
| Worker | A requests a grant for Agent B: read-only Euphony access on B's participation, without further delegation. The issuing authority checks A's actual delegation boundary. |
| Provider call | B's concrete read request passes the selected grant, governing policy and workspace/operation requirements; the Linear Resource uses its shared provider identity. |
| Forbidden broadening | B cannot create write authority or redelegate merely by submitting wider YAML or calling the management tool. |
| A leaves or loses administration rights | B retains its own grant under current policy. A cannot administer another grant without current authority. B's grant can be revoked by another authorized administrator. |
| Root replaced | The new Interface establishes valid access to the existing policy; it does not copy grants out of the old template. |

Every issuer may also be a protected target in another operation. Agent-to-Agent controls, Pool
provisioning and Resource dependency calls follow the same request/basis/check/result structure.

## 8. Review sequence and conformance evidence

Continue within I4, not a new decision ledger:

**Accepted positive authority:** ordinary managed access requires a recorded participation grant,
including the root's broad management access. Policy governs issuance/current use; it does not
provide a grantless alternative. A grant covers many requests and can be established as part of
authorized connection setup.

**Accepted administration/lifetime:** current authority is checked for each administration
operation. Issued grants have no live dependency on issuer history/permissions. Authorized
withdrawal and recipient participation/Conduit lifetime govern their access, subject to current
policy/requirements. No separate ancestry record is required.

1. **Policy/Requirement relationship:** review the [shared-machinery example](#policy-and-requirement-reuse--proposed-refinement).
   Recommend Policy as an authoritative composition of reusable requirements plus grant-administration
   rules, using the existing contract/plugin mechanisms. This refines the still-open portable public
   policy contract; it does not select YAML fields, a new expression language or an evaluator.
2. **Policy composition and failure:** define applicability, rule interaction, incomplete information
   and trustworthy decision inputs under existing whole-path restrictions. Administration boundaries
   for custom constraints need defined checks; a general solver is not assumed.
3. **Placement and persistence:** settle the management Resource capability, first authority and
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
- Replace/stop the issuer and withdraw its administration rights while the recipient retains its
  own grant. Then revoke that grant through a different authorized Interface.
- Withdraw access, sever policy reachability and trigger a required-check failure; show distinct
  evidence while none authorizes a new affected operation.
- Prove an existing provider credential cannot substitute for the requesting Agent's grant.
- Trace a decision to actual endpoint enforcement; include a malicious direct invocation that
  bypasses discovery but is still refused under the same restriction.

These are proposed acceptance cases, not executed tests or authorization to start implementation.
The first proof should establish the common contract using a small implementation before investing
in helper libraries, a new policy DSL or distributed authorization infrastructure.
