# Policy and Grant contracts

Status: accepted semantic model with remaining I4 details, 2026-11-01. Policy and Grant are
essential concepts beneath the five object families, for Agents and Interfaces alike.
Independent policy, recorded participation grants and issuer-independent grant lifetime are
accepted. Policy is now the authoritative application of reusable requirements, and grant
administration uses ordinary protected capabilities. This document develops that model without
selecting exact YAML/MCP fields, an evaluator, credential format or storage engine. New choices below are proposals;
the [design](../../DESIGN.md) retains accepted behavior and [I4](REVIEW.md#i4--local-transactional-authority)
is the ruling queue. [Research](../../research/AUTHORIZATION.md) provides sources and alternatives.

## 1. Place in the model

| Concept | Responsibility | What it does not imply |
| --- | --- | --- |
| Policy | Authoritative application of reusable requirements to applicable actors, targets or relationships. | A separate executable rule category, permission language, running instance or grant merely because a file was authored. |
| Grant | A particular issued authorization for an identified subject, target/capability and permitted extent, with its own participation lifetime and authorized administration. | A bearer secret, an OAuth token, a parent-dependent authority chain, or the union of unrelated access paths. |
| Requirement | A reusable typed guarantee with applicability, timing and endpoint duties. Policies/grants can reuse applicable restrictions. | All requirements being authorization rules, or satisfying a restriction creating positive authority. |
| Plugin | Contracts and implementation bindings for management, evaluation and enforcement operations. | Every plugin becoming an issuer or implementing its own permission server. |
| Operation / graph | A callable action / composition of actions, including protected grant administration and governing-binding changes. | An executable declaration granting itself authority or freezing authorization for all later graph nodes. |
| Conduit | An instantiated relationship carrying concrete participation, routes, applicable requirements and access evidence. | Ownership of every governing policy or independent invention of permission. |

Policy and Grant are protocol concepts with data/lifecycle contracts. They do not extend the five
families or need their own template-to-running-instance hierarchy. Published definitions, authoritative
requirement bindings and issued grants must remain distinguishable. Installing a plugin or expanding a
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

## 3. Policy as authoritative requirement application

**Accepted model:** Policy means the authoritative application of reusable Requirement contracts.
Grant administration is an ordinary protected capability governed by grants and requirements.
There is no separate policy-rule language, grant-administration rule category or mandatory policy
plugin process. This is a portable semantic boundary; exact data/wire schemas and engine choices
remain open.

The model distinguishes reusable definitions, their application and issued authority:

| Part | Responsibility |
| --- | --- |
| Requirement definition/composition | Versioned meaning, typed parameters, applicability, support and enforcement duties. Composition retains each constituent's guarantees. |
| Governing requirement binding | Authoritative state recording what requirements apply to which actors, targets or relationships. Policy names this governed application; it remains independent of the administering Interface. |
| Relationship requirements | Requirements on a particular Conduit and its use. They may narrow access or require behavior without granting authority or becoming global restrictions. |
| Grant | Actual issued authority on a participant's selected path, with its own access extent and lifetime. |
| Administrative capability | Operations to administer grants and governing requirement bindings, protected by the same grant/requirement model as other operations. |

An administrator may apply a workspace restriction to a Resource, while a Conduit adds a smaller
result bound. The same requirement definitions and selected supporting implementations can serve
both. A template that requests or composes requirements does not activate governing rules or
issue itself a grant. Installing a plugin also supplies no authority.

### Governing state and composition

Independent policy state remains necessary even without a separate rule language. The responsible
authority must retain current bindings and coordinate checked changes. An authorized Interface or
Agent administers that state through its permitted capabilities; it does not own the state merely
because it requested the change. Replacing an Interface does not reset it.

**Accepted explicit targeting:** bindings distinguish the target object's own operations from
operations on other objects selected through ownership. A Pool provisioning restriction does not
implicitly restrict existing Resources' deployment operations. An explicit owned-Resource scope
can select those supported capabilities without separately authoring every Resource's rule.
Pool templates may bundle both declarations; authoring location does not determine applicability.
V0 does not automatically propagate Pool requirements. The access-management Resource's role as
record manager likewise does not make it the target of every binding it stores.

For example, approved-template requirements can constrain Euphony Pool's provisioning capability;
allowed-region requirements can separately target deployment capabilities of its owned Resources.
A repository-wide read-only requirement applies across relevant Conduits to that repository, while
a read-only restriction on one review Conduit stays local to that path. In each case the declared
requirement contract must support the target/context, and the caller still needs its grant.

**Accepted live matching:** an explicit ownership-based governing selection uses current matching
Resource instances at applicable checks. A newly matching instance becomes subject to the binding;
one that stops matching leaves that binding's reach. Other requirements and grants retain their
own rules. This neither provisions Resources nor grants access. Explicit references stay bound to
their resolved stable IDs; matching is not automatic Pool inheritance or substitution of a deleted
referenced target. Current-authority and no-offline-grants checks still apply.

All applicable requirements must hold at their declared enforcement points. Actor-wide and
target-wide restrictions apply wherever their declared applicability requires; Conduit-local
restrictions remain local to that path. Separate authorized paths stay complete alternatives, and
operation/target/parameter bounds cannot be combined across paths to fabricate broader access.
Known-conflict/combined-effect checks are optional; there is no required general solver. A required
check that cannot be established blocks the affected behavior. These are existing accepted rules,
not a new policy-specific precedence system.

A policy change can affect subsequent managed requests under the accepted live-restriction rules.
It cannot silently install an evaluator, replace endpoint code or drop unsupported requirements.
The exact applicability/binding schema, management operation granularity and retained versions
remain for review. No automatic Pool inheritance, generic label selectors or separate
permit/deny/exception language is introduced. A proposed binding inventory is:

| Concern | Meaning, with exact fields still open |
| --- | --- |
| Authoritative identity/revision | Identify the managed state and protect checked changes; the unit of binding updates is not selected here. |
| Applicability | Explicit actors, targets, capabilities, relationships or operations to which the binding applies. |
| Requirement references and values | Known versioned definitions/compositions with validated parameters and declared context support. |
| Current availability and operation evidence | Resolve current authority and report checked management outcomes without a grant-ancestry dependency. |

### Grant administration uses ordinary capabilities

The administrator needs a recorded grant permitting the requested administration operation.
Applicable requirements constrain that operation's proposed effect: which target/recipient may
receive a grant, which operations and parameter bounds it may contain, and whether it may include
further administrative access. Read access to a service alone does not grant administration.

Issuance, changes and revocation all check current authority. A successful issuance records the
recipient's own grant. That grant does not depend on the issuer's later rights, execution or
participation. Another currently authorized administrator can revoke it. The accepted initial
owner authority still needs a startup contract; presenting an arbitrary grant-issuance plugin
cannot establish root authority.

**Context support matters:** checking a provider request and checking a proposed grant's permitted
extent are different tasks. Shared requirement definitions do not make a per-request checker a
general subset prover. Implementations must support the relevant use; unverifiable bounds cannot
be assumed satisfied. Detailed type-specific validation contracts belong to the authoring/conformance
pass rather than requiring a new universal comparison language.

### Carrying use bounds into issued grants

**Accepted:** bounded grant-issuance operations may supply their mandatory use restrictions in the
grant they create, with additional caller-supplied restrictions composing conjunctively. They return
the complete actual issued extent for inspection. The caller still selects the intended recipient,
Resource/participation and requested access under the operation's typed contract; omitted restrictions
do not mean granting everything the administrator could issue. This uses the accepted
requirement-composition machinery and does not require a generic proof that an arbitrary proposed
condition is equivalent to, or narrower than, another condition.

For example, A's administrative capability may issue read access to Euphony's Linear Resource.
The handler includes the permitted operation/target/workspace bounds in B's grant even if A omits
their repeated declaration. A request can add a smaller result bound. An explicitly requested
operation or target outside the permitted extent is refused rather than silently reported as
issued; known conflicting requirements are reported. Unknown comparisons or unsupported required
checks cannot be treated as satisfied.

Only restrictions defined to govern subsequent use are carried into the grant, with the concrete
values and referenced contract versions that give them meaning. Checks declared only for issuance,
such as the issuer's current administrative authority, do not become ongoing grant dependencies.
Requirements retain their declared applicability, timing and duties; current recipient/target
restrictions continue to apply where their contracts require them.
A provider-request checker does not automatically support checking a proposed grant. Relevant
support remains contract-declared and verified, and custom administration checks may still be needed.

B's retained bounds are its own authorization data, not a link to A's grant or an issuer-provenance
chain. A later losing permission does not change B's grant. Current governing requirements still
apply, and an authorized administrator can change or revoke B's grant through the existing model.

A strict operation accepting a complete caller-supplied grant may also be offered, with the same
authority and boundary checks. It is not mandatory for every caller to repeat each restriction.
Neither operation style grants permission from requirement satisfaction alone or replaces current
authority checks. Exact operation input/output schemas and specific validation support stay for I7
and conformance work.

### Concrete examples

**Shared Linear access:** Euphony Pool owns an authenticated Linear Resource. Coordinator A has
a grant for the management capability's grant-issuance operation, constrained to selected recipients,
that Resource, permitted read operations and Euphony workspace access. A requests Worker B's access;
the administration operation checks these requirements and records B's grant on B's participation.
B's provider calls require that grant and all applicable governing/Conduit requirements. A later
losing its administration rights does not revoke B. Another authorized administrator can revoke B
through the corresponding protected capability. Native provider operation names remain intact.

**Managed deployments:** an administrator applies an allowed-region requirement to a deployment
Resource, permitting two regions. One Agent's Conduit narrows that to one. The Agent needs a
deployment grant, and the selected endpoint enforces both requirements before the managed effect.
Another Conduit cannot bypass the Resource-wide restriction. This does not imply automatic Pool
inheritance or interception of arbitrary native CLI calls.

**Retained messages:** a Pool-owned exchange Resource supplies shared append-only history.
Participants have read/post grants. Their Conduit requires retained delivery and bounded catch-up.
Access restrictions and delivery guarantees use common typed requirement machinery with different
duties: plugins check permissions and also provide the promised persistence/delivery behavior.
Successful boolean evaluation alone cannot supply that behavior.

### Common extension machinery

One plugin package can publish requirement definitions, compositions, evaluation operations and
endpoint enforcement support. Separate implementations may support the same versioned meaning.
There is no required policy-specific package, Resource per rule or process per check. Policies,
grant bounds and Conduit filters reuse applicable contracts rather than duplicate their schemas.

The operation handler and responsible endpoint still fulfill their declared duties: reject a
forbidden dispatch, enforce a result bound before delivery, or retain message history. Graphs can
invoke supporting operations but cannot omit baseline authorization or turn a setup-time check
into lasting permission. Relationship changes retain their supported-control/fixed-definition
rules; governing bindings use authorized checked management operations.

## 4. Proposed Grant structure

| Part | Meaning |
| --- | --- |
| Identity and revision | Stable issued record and checked management version. The ID is a reference, not a credential. |
| Recipient | The identified actor/subject receiving authority. Human principal and Interface endpoint distinctions remain explicit where needed. |
| Target and access extent | Concrete targets/capabilities, permitted operations and associated parameter/resource restrictions, preserved together. |
| Participation/path binding | The logical attachment or other declared authorization context in which the grant may be exercised. Service-access grants already follow recipient participation. |
| Administration evidence | A link to normal operation evidence where retained; no separate grant-provenance chain is required for authorization. |
| Current governing restrictions | Applicable policy and requirements constrain use independently of the grant's issuer/history; this is not a parent-grant dependency. |
| Administrative operation bounds, where applicable | For a grant permitting administrative operations, requirements constrain proposed grants/bindings. Ordinary service-use permission does not imply administration; no separate delegation-rule language is introduced. |
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

**Accepted placement:** the system Pool owns a dedicated access-management Resource for grant
records and governing requirement bindings. Its capabilities expose authorized administration and
inspection through Conduits to Interfaces and permitted Agents. The existing system management
Resource retains template, registration and instance-creation responsibilities. This separates
unitary responsibilities without adding an object family.

Both Resources may run in the same management process and use the same internal storage
implementation. There is no required process/database per Resource, Resource per rule, or central
proxy for every provider call. Management retains authoritative access state; endpoint-selected
plugins still fulfill their enforcement duties. Policy remains independent of its administrative
Interface, and one capability manages many records. Exact capability/type names, storage and
startup wiring remain open.

The initial trusted authority must be selected by the system's establishment contract. An arbitrary
Resource advertising policy evaluation cannot approve its own use or confer root authority.
Explicit first initialization uses verified local-owner authority until ordinary management grants
are established. Explicit resume/repair uses the root Interface's declared recovery operations,
with runtime owner verification even when ordinary management access is broken. Recognition details
and first-record/recovery schemas remain I7 or implementation work; initialization does not become
an ordinary grantless access path.
Policy records are not ordinary knowledge notes: copying, searching or sharing a note cannot confer
administrative authority or bypass the policy-management contract.

Proposed operation responsibilities, with names deferred:

| Surface | Responsibilities |
| --- | --- |
| Administration | Inspect governing requirement bindings/grants, submit checked binding changes, issue/narrow/withdraw grants and inspect their outcomes. These are ordinary protected capability operations. |
| Evaluation | Resolve current authoritative access facts for a concrete request and return the typed decision/evidence. |
| Enforcement | Bind authenticated identity/path, apply declared checks at dispatch/delivery and record outcomes. |
| Observation | Explain current grants, governing restrictions, failed checks and incomplete coverage through the Interface. |

An ordinary operation plugin author declares operations and requirement duties and consumes trusted
invocation context; the runtime supplies common authorization plumbing. Selected plugins implement
the requirement checks and protected management operations; no separate policy-plugin class is needed.
Graphs call operations with current authority
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

**Accepted consolidation:** Policy is the [authoritative application of requirements](#3-policy-as-authoritative-requirement-application).
Grant administration uses ordinary protected capabilities. Existing composition, mandatory-check
and complete-path rules apply without a second policy language or grant-administration rule category.

**Accepted placement:** a dedicated access-management Resource in the system Pool manages grants
and governing requirement bindings, alongside the existing registry/template management Resource.
Logical separation does not require separate processes, stores or centralized endpoint enforcement.

**Accepted target scope:** explicit bindings distinguish an object's own operations from selected
owned Resources. Pool templates may bundle those rules, but ownership/authoring location alone
does not propagate requirements. No automatic Pool inheritance is introduced.

**Accepted live target matching:** explicit ownership-based governing selections match live at
subsequent applicable checks. Newly matching Resource instances become subject to the binding;
nonmatching instances leave its reach. This does not provision Resources or issue grants, and
explicit references retain their resolved stable IDs. Other applicable restrictions remain in force.
For example, a binding targeting Euphony's deployment Resources also applies to a newly provisioned
matching `worker-deploy`, while an unrelated directory Resource does not match. Exact selectors
and membership-resolution mechanisms remain I7/implementation work.

**Accepted grant bounds:** [bounded issuance operations supply mandatory use restrictions](#carrying-use-bounds-into-issued-grants)
and compose additional restrictions, returning the complete grant. They preserve requested intent,
refuse explicit broadening and do not create a live issuer dependency. Strict complete-proposal
operations remain possible. Detailed schemas and type-specific checks stay for I7/conformance.

**Accepted first-launch arrangement:** the runtime loads a locally available root Interface
definition before registry services exist, with initial owner recognition and minimum
loading/persistence support. Its graph directs establishment of the system Pool and management
Resources. During explicit first initialization, the runtime authorizes that setup from the verified
local-owner context. Once access management is ready, setup establishes ordinary recorded management
grants and subsequent administration uses normal managed paths. This first-authorization boundary
is accepted; initialization authority must not remain an application bypass or revive automatically
after an outage.

**Accepted explicit recovery:** resuming partial setup or repairing a broken management
path through the locally loadable root Interface's declared recovery operations, authorized by
verified local-owner context when normal access management is unavailable, is the selected route.
Recovery preserves known identities, data and outcomes and reconciles uncertain effects. This does
not reopen initialization automatically or authorize ordinary application requests offline.

**Next area — persistence:** choose whether authority storage has a published plugin contract in
v0 or remains behind internal service interfaces initially, then settle its representation/lifecycle
and first engine. These choices must preserve the accepted ownership and recovery contracts.

The final I7 pass chooses policy/grant schemas, package publication, references, operation names,
dynamic MCP-operation matching and retained implementation bindings. Registering new contract
types and activating governing requirement bindings remain different operations. There is no
separate public policy-programming language; no Cedar/OPA/token engine is selected.

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
